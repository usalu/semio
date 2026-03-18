// #region 🔖Header
// [👤semio🖱️vscode💻extension](semiorepo://p/u/semio/b/u/vscode/f/extension.ts)

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

// Specs: VS Code extension that opens kit.json files in an embedded sketchpad webview editor.
// Uses CustomTextEditorProvider to bridge between VS Code filesystem and sketchpad UI.

// VS Code extension providing a sketchpad-based custom editor for semio kit JSON files.

// #endregion 🔖Header

// #region 🔖Imports
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
// #endregion 🔖Imports

// #region 🔖KitEditor
// [👤semio🖱️vscode💻extension🔖kiteditor](semiorepo://p/u/semio/b/u/vscode/f/extension.ts/s/Kit%20Editor)
// Kit editor MUST provide a custom editor for kit JSON files using the sketchpad webview.
// Specs: Opens kit.json files in a webview panel that loads the built sketchpad app.
// File changes are bridged between the VS Code filesystem and the webview via messaging.

/**
 * Custom editor provider that renders kit JSON files using the sketchpad webview.
 * [👤semio🖱️vscode💻extension🔖kiteditor🪨kiteditorprovider](semiorepo://p/u/semio/b/u/vscode/f/extension.ts/s/Kit%20Editor/d/i/KitEditorProvider)
 *
 * Specs: Implements VS Code CustomTextEditorProvider. Loads the sketchpad app in a webview
 * and bridges file reads/writes via postMessage.
 **/
class KitEditorProvider implements vscode.CustomTextEditorProvider {
  public static readonly viewType = "semio.kitEditor";

  constructor(private readonly context: vscode.ExtensionContext) {}

  public async resolveCustomTextEditor(
    document: vscode.TextDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken,
  ): Promise<void> {
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
    webviewPanel.webview.html = html;

    const updateWebview = () => {
      webviewPanel.webview.postMessage({
        kind: "kit.update",
        content: document.getText(),
      });
    };

    const changeDocumentSubscription = vscode.workspace.onDidChangeTextDocument((e) => {
      if (e.document.uri.toString() === document.uri.toString()) {
        updateWebview();
      }
    });

    webviewPanel.onDidDispose(() => {
      changeDocumentSubscription.dispose();
    });

    webviewPanel.webview.onDidReceiveMessage((message) => {
      if (message.kind === "kit.save") {
        const edit = new vscode.WorkspaceEdit();
        edit.replace(
          document.uri,
          new vscode.Range(0, 0, document.lineCount, 0),
          message.content,
        );
        vscode.workspace.applyEdit(edit);
      }
    });

    updateWebview();
  }

  private getFallbackHtml(document: vscode.TextDocument): string {
    const content = document.getText();
    let kitName = "Kit";
    try {
      const parsed = JSON.parse(content);
      if (parsed?.name) kitName = parsed.name;
    } catch { /* ignore parse errors */ }
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
// [👤semio🖱️vscode💻extension🔖activation](semiorepo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation)
// MUST register the custom editor provider on activation.

/**
 * Activates the semio VS Code extension.
 * [👤semio🖱️vscode💻extension🔖activation🛠️activate](semiorepo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation/d/i/activate)
 **/
export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.window.registerCustomEditorProvider(
      KitEditorProvider.viewType,
      new KitEditorProvider(context),
      { webviewOptions: { retainContextWhenHidden: true } },
    ),
  );
}

/**
 * Deactivates the semio VS Code extension.
 * [👤semio🖱️vscode💻extension🔖activation🛠️deactivate](semiorepo://p/u/semio/b/u/vscode/f/extension.ts/s/Activation/d/i/deactivate)
 **/
export function deactivate() {}
// #endregion 🔖Activation
