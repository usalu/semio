// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Specs: VS Code extension that opens compose kit JSON files in an embedded sketchpad webview editor.
// Uses CustomTextEditorProvider to bridge between VS Code filesystem and sketchpad UI.
// The webview loads the bundled sketchpad app or the sibling workspace sketchpad build and receives kit JSON via postMessage.
// File writes flow back from the webview to the VS Code workspace API.

// VS Code extension providing a sketchpad-based custom editor for compose kit JSON files.

// #endregion 🧲Header

// #region 🔌Adapters
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
// #endregion 🔌Adapters

// #region 🛕KitFileDetection
// Kit file detection MUST match the naming conventions used across the compose workspace.
// Specs: Matches `*.kit.json`, `kit_*.json`, `kit-*.json`, and nested `.compose/kit.json` files.
// Detection is path-based so the extension can route known kit files into the custom editor
// without opening unrelated JSON artifacts such as schemas.

/**
 * Returns whether a file path should be treated as a compose kit JSON file.
 *
 * Specs: Normalizes path separators and matches the workspace kit filename conventions.
 * Supports `*.kit.json`, `kit_*.json`, `kit-*.json`, and nested `.compose/kit.json` files.
 **/
export function isLikelyKitJsonFilePath(filePath: string): boolean {
  const normalizedFilePath = filePath.replace(/\\/g, "/").toLowerCase();
  const baseName = path.posix.basename(normalizedFilePath);

  if (normalizedFilePath.endsWith("/.compose/kit.json")) return true;
  if (baseName.endsWith(".kit.json")) return true;
  if (/^kit[_-].+\.json$/u.test(baseName)) return true;
  return false;
}
// #endregion 🛕KitFileDetection

// #region 🌈SketchpadDist
// Sketchpad asset resolution MUST support both packaged extensions and local development.
// Specs: The extension first prefers bundled `sketchpad-dist`, then falls back to the
// workspace sketchpad build at `../sketchpad/dist`. Resolution succeeds only when webview.html exists.
// The resolved folder is exposed for test coverage and webview localResourceRoots setup.

/**
 * Returns the candidate sketchpad dist folders for the current extension path.
 *
 * Specs: Candidate order prefers the extension-bundled dist and then the sibling workspace dist.
 * Returned paths are absolute and not filtered for existence.
 **/
export function getSketchpadDistCandidatePaths(extensionPath: string): string[] {
  return [path.join(extensionPath, "sketchpad-dist"), path.resolve(extensionPath, "..", "sketchpad", "dist")];
}

/**
 * Resolves the first usable sketchpad dist folder.
 *
 * Specs: A folder is usable only when `webview.html` exists inside it.
 * Returns null when neither the bundled nor sibling workspace dist is available.
 **/
export function resolveSketchpadDistPath(extensionPath: string): string | null {
  for (const candidatePath of getSketchpadDistCandidatePaths(extensionPath)) {
    if (fs.existsSync(path.join(candidatePath, "webview.html"))) {
      return candidatePath;
    }
  }

  return null;
}
// #endregion 🌈SketchpadDist

// #region ⏲️MessageProtocol
// Message protocol between extension host and sketchpad webview.
// Specs: Messages use a `kind` discriminator. Extension sends kit data to webview.
// Webview sends save requests back. The protocol is intentionally thin — the
// webview manages its own in-memory KitStore and only communicates on file I/O boundaries.

/**
 * Messages from extension host to webview.
 **/
type ExtensionToWebviewMessage = { kind: "kit.load"; content: string } | { kind: "kit.externalUpdate"; content: string };

/**
 * Messages from webview to extension host.
 **/
type WebviewToExtensionMessage = { kind: "kit.save"; content: string } | { kind: "kit.ready" };

// #endregion ⏲️MessageProtocol

// #region 📜KitEditor
// Kit editor MUST provide a custom editor for compose kit JSON files using the sketchpad webview.
// Specs: Opens known compose kit file conventions in a webview panel that loads the sketchpad app.
// File changes are bridged between the VS Code filesystem and the webview via messaging.
// External file changes (e.g., from git or another editor) trigger kit.externalUpdate.

/**
 * Custom editor provider that renders kit JSON files using the sketchpad webview.
 *
 * Specs: Implements VS Code CustomTextEditorProvider. Loads the sketchpad app in a webview
 * and bridges file reads/writes via postMessage. Watches for external file changes.
 * The webview creates a JsonFileKitStore internally using a message-based adapter.
 **/
class KitEditorProvider implements vscode.CustomTextEditorProvider {
  public static readonly viewType = "compose.kitEditor";

  constructor(private readonly context: vscode.ExtensionContext) { }

  public async resolveCustomTextEditor(document: vscode.TextDocument, webviewPanel: vscode.WebviewPanel, _token: vscode.CancellationToken): Promise<void> {
    const sketchpadDistPath = resolveSketchpadDistPath(this.context.extensionPath);

    webviewPanel.webview.options = {
      enableScripts: true,
      localResourceRoots: sketchpadDistPath ? [vscode.Uri.file(sketchpadDistPath), vscode.Uri.file(this.context.extensionPath)] : [vscode.Uri.file(this.context.extensionPath)],
    };

    if (!sketchpadDistPath) {
      webviewPanel.webview.html = this.getFallbackHtml(document);
      return;
    }

    const indexHtmlPath = path.join(sketchpadDistPath, "webview.html");
    let html = fs.readFileSync(indexHtmlPath, "utf-8");
    const baseUri = webviewPanel.webview.asWebviewUri(vscode.Uri.file(sketchpadDistPath));
    html = html.replace(/<head>/, `<head><base href="${baseUri.toString()}/">`);
    html = html.replace(/src="\//g, `src="${baseUri.toString()}/`);
    html = html.replace(/href="\//g, `href="${baseUri.toString()}/`);

    // Inject the kit data loading script before closing </body> tag.
    // 🆕The webview script will listen for messages and create a JsonFileKitStore.
    const kitBootScript = `
<script>
  (function() {
    // Bridge between VS Code extension and sketchpad webview.
    // The webview receives kit JSON via postMessage and uses it
    // 🔁to initialize/update an in-memory kit store.
    const vscode = acquireVsCodeApi();

    // Store the initial kit content for the sketchpad to pick up.
    window.__COMPOSE_KIT_JSON__ = ${JSON.stringify(document.getText())};
    window.__COMPOSE_VSCODE_API__ = vscode;

    // Listen for external updates from the extension host.
    window.addEventListener('message', function(event) {
      const message = event.data;
      if (message.kind === 'kit.externalUpdate') {
        window.__COMPOSE_KIT_JSON__ = message.content;
        if (window.__COMPOSE_ON_EXTERNAL_UPDATE__) {
          window.__COMPOSE_ON_EXTERNAL_UPDATE__(message.content);
        }
      }
    });
  })();
</script>`;

    html = html.replace(/<\/head>/, `${kitBootScript}\n</head>`);
    webviewPanel.webview.html = html;

    // 🔷Track whether we are applying our own edit to avoid feedback loops.
    let isApplyingEdit = false;

    // ♻️Listen for document changes (from external sources like git, other editors).
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
<p>The sketchpad app was not found in <code>sketchpad-dist/</code> or <code>../sketchpad/dist/</code>. Build it with <code>npx nx build @semio-tech/compose-sketchpad</code>.</p>
<pre>${content.slice(0, 5000)}</pre>
</body></html>`;
  }
}
// #endregion 📜KitEditor

// #region 🏷️Activation
// MUST register the custom editor provider on activation.

/**
 * Activates the compose VS Code extension.
 **/
export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(vscode.window.registerCustomEditorProvider(KitEditorProvider.viewType, new KitEditorProvider(context), { webviewOptions: { retainContextWhenHidden: true } }));
}

/**
 * Deactivates the compose VS Code extension.
 **/
export function deactivate() { }
// #endregion 🏷️Activation
