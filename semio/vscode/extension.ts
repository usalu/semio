// #region 🔖Header
// [👤semio🖱️vscode💻extension](repo://p/u/semio/b/u/vscode/f/extension.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Specs: VS Code extension that opens *.kit.json files in an embedded sketchpad webview editor.
// Uses CustomTextEditorProvider to bridge between VS Code filesystem and sketchpad UI.
// The webview loads the built sketchpad app and receives kit JSON via postMessage.
// File writes flow back from the webview to the VS Code workspace API.

// VS Code extension providing a sketchpad-based custom editor for semio kit JSON files.

// #endregion 🔖Header

// #region 🔖Imports
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
// #endregion 🔖Imports

// #region 🔖MessageProtocol
// [👤semio🖱️vscode💻extension🔖messageprotocol](repo://p/u/semio/b/u/vscode/f/extension.ts/s/MessageProtocol)
// Message protocol between extension host and sketchpad webview.
// Specs: Messages use a `kind` discriminator. Extension sends kit data to webview.
// Webview sends save requests back. The protocol is intentionally thin — the
// webview manages its own in-memory KitStore and only communicates on file I/O boundaries.

/**
 * Messages from extension host to webview.
 * [👤semio🖱️vscode💻extension🔖messageprotocol🛠️extensiontowebviewmessage](repo://p/u/semio/b/u/vscode/f/extension.ts/s/MessageProtocol/d/i/ExtensionToWebviewMessage)
 **/
type ExtensionToWebviewMessage = { kind: "kit.load"; content: string } | { kind: "kit.externalUpdate"; content: string };

/**
 * Messages from webview to extension host.
 * [👤semio🖱️vscode💻extension🔖messageprotocol🛠️webviewtoextensionmessage](repo://p/u/semio/b/u/vscode/f/extension.ts/s/MessageProtocol/d/i/WebviewToExtensionMessage)
 **/
type WebviewToExtensionMessage = { kind: "kit.save"; content: string } | { kind: "kit.ready" };

// #endregion 🔖MessageProtocol

// #region 🔖KitEditor
// [👤semio🖱️vscode💻extension🔖kiteditor](repo://p/u/semio/b/u/vscode/f/extension.ts/s/KitEditor)
// Kit editor MUST provide a custom editor for *.kit.json files using the sketchpad webview.
// Specs: Opens *.kit.json files in a webview panel that loads the built sketchpad app.
// File changes are bridged between the VS Code filesystem and the webview via messaging.
// External file changes (e.g., from git or another editor) trigger kit.externalUpdate.

/**
 * Custom editor provider that renders kit JSON files using the sketchpad webview.
 * [👤semio🖱️vscode💻extension🔖kiteditor🪨kiteditorprovider](repo://p/u/semio/b/u/vscode/f/extension.ts/s/KitEditor/d/i/KitEditorProvider)
 *
 * Specs: Implements VS Code CustomTextEditorProvider. Loads the sketchpad app in a webview
 * and bridges file reads/writes via postMessage. Watches for external file changes.
 * The webview creates a JsonFileKitStore internally using a message-based adapter.
 **/
class KitEditorProvider implements vscode.CustomTextEditorProvider {
  public static readonly viewType = "semio.kitEditor";

  constructor(private readonly context: vscode.ExtensionContext) { }

  public async resolveCustomTextEditor(document: vscode.TextDocument, webviewPanel: vscode.WebviewPanel, _token: vscode.CancellationToken): Promise<void> {
    webviewPanel.webview.options = {
      enableScripts: true,
    };

    const sketchpadDistPath = path.join(this.context.extensionPath, "sketchpad-dist");
    const indexHtmlPath = path.join(sketchpadDistPath, "index.html");

    if (!fs.existsSync(indexHtmlPath)) {
      webviewPanel.webview.html = this.getFallbackHtml(document);
      return;
    }

    let html = fs.readFileSync(indexHtmlPath, "utf-8");
    const baseUri = webviewPanel.webview.asWebviewUri(vscode.Uri.file(sketchpadDistPath));
    html = html.replace(/<head>/, `<head><base href="${baseUri.toString()}/">`);
    html = html.replace(/src="\//g, `src="${baseUri.toString()}/`);
    html = html.replace(/href="\//g, `href="${baseUri.toString()}/`);

    // Inject the kit data loading script before closing </body> tag.
    // The webview script will listen for messages and create a JsonFileKitStore.
    const kitBootScript = `
<script>
  (function() {
    // Bridge between VS Code extension and sketchpad webview.
    // The webview receives kit JSON via postMessage and uses it
    // to initialize/update an in-memory kit store.
    const vscode = acquireVsCodeApi();

    // Store the initial kit content for the sketchpad to pick up.
    window.__SEMIO_KIT_JSON__ = ${JSON.stringify(document.getText())};
    window.__SEMIO_VSCODE_API__ = vscode;

    // Listen for external updates from the extension host.
    window.addEventListener('message', function(event) {
      const message = event.data;
      if (message.kind === 'kit.externalUpdate') {
        window.__SEMIO_KIT_JSON__ = message.content;
        if (window.__SEMIO_ON_EXTERNAL_UPDATE__) {
          window.__SEMIO_ON_EXTERNAL_UPDATE__(message.content);
        }
      }
    });
  })();
</script>`;

    html = html.replace(/<\/head>/, `${kitBootScript}\n</head>`);
    webviewPanel.webview.html = html;

    // Track whether we are applying our own edit to avoid feedback loops.
    let isApplyingEdit = false;

    // Listen for document changes (from external sources like git, other editors).
    const changeDocumentSubscription = vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.uri.toString() === document.uri.toString() && !isApplyingEdit) {
        webviewPanel.webview.postMessage({
          kind: "kit.externalUpdate",
          content: document.getText(),
        } satisfies ExtensionToWebviewMessage);
      }
    });

    webviewPanel.onDidDispose(() => {
      changeDocumentSubscription.dispose();
    });

    // Handle messages from the webview.
    webviewPanel.webview.onDidReceiveMessage((message: WebviewToExtensionMessage) => {
      if (message.kind === "kit.save") {
        isApplyingEdit = true;
        const edit = new vscode.WorkspaceEdit();
        edit.replace(document.uri, new vscode.Range(0, 0, document.lineCount, 0), message.content);
        vscode.workspace.applyEdit(edit).then(
          () => {
            isApplyingEdit = false;
          },
          () => {
            isApplyingEdit = false;
          },
        );
      }
    });
  }

  private getFallbackHtml(document: vscode.TextDocument): string {
    const content = document.getText();
    let kitName = "Kit";
    try {
      const parsed = JSON.parse(content);
      if (parsed?.name) kitName = parsed.name;
    } catch {
      /* ignore parse errors */
    }
    return `<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>${kitName}</title>
<style>body{font-family:system-ui;padding:2em;color:#333}pre{background:#f5f5f5;padding:1em;border-radius:4px;overflow:auto}</style>
</head>
<body>
<h1>Kit: ${kitName}</h1>
<p>The sketchpad app is not bundled. Build with <code>npx nx build @semio/sketchpad</code> and copy dist to <code>sketchpad-dist/</code>.</p>
<pre>${content.slice(0, 5000)}</pre>
</body></html>`;
  }
}
// #endregion 🔖KitEditor

// #region 🔖Activation
// [👤semio🖱️vscode💻extension🔖activation](repo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation)
// MUST register the custom editor provider on activation.

/**
 * Activates the semio VS Code extension.
 * [👤semio🖱️vscode💻extension🔖activation🛠️activate](repo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation/d/i/activate)
 **/
export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(vscode.window.registerCustomEditorProvider(KitEditorProvider.viewType, new KitEditorProvider(context), { webviewOptions: { retainContextWhenHidden: true } }));
}

/**
 * Deactivates the semio VS Code extension.
 * [👤semio🖱️vscode💻extension🔖activation🛠️deactivate](repo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation/d/i/deactivate)
 **/
export function deactivate() { }
// #endregion 🔖Activation
