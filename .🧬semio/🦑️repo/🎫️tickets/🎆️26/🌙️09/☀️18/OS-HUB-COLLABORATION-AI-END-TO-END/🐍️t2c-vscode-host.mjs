/** 🧩️ Constructible stand-ins for every `vscode.*` member the extension reaches for. */
export class CancellationToken { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(CancellationToken, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : CancellationToken), apply: () => CancellationToken, construct: () => new CancellationToken() }));
export class CodeLens { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(CodeLens, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : CodeLens), apply: () => CodeLens, construct: () => new CodeLens() }));
export class CodeLensProvider { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(CodeLensProvider, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : CodeLensProvider), apply: () => CodeLensProvider, construct: () => new CodeLensProvider() }));
export class Command { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Command, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Command), apply: () => Command, construct: () => new Command() }));
export class DecorationOptions { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(DecorationOptions, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : DecorationOptions), apply: () => DecorationOptions, construct: () => new DecorationOptions() }));
export class Diagnostic { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Diagnostic, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Diagnostic), apply: () => Diagnostic, construct: () => new Diagnostic() }));
export class DiagnosticCollection { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(DiagnosticCollection, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : DiagnosticCollection), apply: () => DiagnosticCollection, construct: () => new DiagnosticCollection() }));
export class DiagnosticSeverity { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(DiagnosticSeverity, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : DiagnosticSeverity), apply: () => DiagnosticSeverity, construct: () => new DiagnosticSeverity() }));
export class DocumentSymbol { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(DocumentSymbol, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : DocumentSymbol), apply: () => DocumentSymbol, construct: () => new DocumentSymbol() }));
export class EventEmitter { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(EventEmitter, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : EventEmitter), apply: () => EventEmitter, construct: () => new EventEmitter() }));
export class ExtensionContext { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(ExtensionContext, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : ExtensionContext), apply: () => ExtensionContext, construct: () => new ExtensionContext() }));
export class OutputChannel { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(OutputChannel, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : OutputChannel), apply: () => OutputChannel, construct: () => new OutputChannel() }));
export class Position { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Position, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Position), apply: () => Position, construct: () => new Position() }));
export class ProgressLocation { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(ProgressLocation, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : ProgressLocation), apply: () => ProgressLocation, construct: () => new ProgressLocation() }));
export class QuickPickItem { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(QuickPickItem, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : QuickPickItem), apply: () => QuickPickItem, construct: () => new QuickPickItem() }));
export class Range { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Range, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Range), apply: () => Range, construct: () => new Range() }));
export class Selection { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Selection, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Selection), apply: () => Selection, construct: () => new Selection() }));
export class SymbolInformation { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(SymbolInformation, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : SymbolInformation), apply: () => SymbolInformation, construct: () => new SymbolInformation() }));
export class SymbolKind { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(SymbolKind, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : SymbolKind), apply: () => SymbolKind, construct: () => new SymbolKind() }));
export class TextDocument { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TextDocument, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TextDocument), apply: () => TextDocument, construct: () => new TextDocument() }));
export class TextEditor { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TextEditor, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TextEditor), apply: () => TextEditor, construct: () => new TextEditor() }));
export class TextEditorDecorationType { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TextEditorDecorationType, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TextEditorDecorationType), apply: () => TextEditorDecorationType, construct: () => new TextEditorDecorationType() }));
export class ThemeIcon { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(ThemeIcon, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : ThemeIcon), apply: () => ThemeIcon, construct: () => new ThemeIcon() }));
export class TreeDataProvider { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TreeDataProvider, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TreeDataProvider), apply: () => TreeDataProvider, construct: () => new TreeDataProvider() }));
export class TreeItem { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TreeItem, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TreeItem), apply: () => TreeItem, construct: () => new TreeItem() }));
export class TreeItemCollapsibleState { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(TreeItemCollapsibleState, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : TreeItemCollapsibleState), apply: () => TreeItemCollapsibleState, construct: () => new TreeItemCollapsibleState() }));
export class Uri { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(Uri, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : Uri), apply: () => Uri, construct: () => new Uri() }));
export class ViewColumn { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(ViewColumn, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : ViewColumn), apply: () => ViewColumn, construct: () => new ViewColumn() }));
export class WorkspaceEdit { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(WorkspaceEdit, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : WorkspaceEdit), apply: () => WorkspaceEdit, construct: () => new WorkspaceEdit() }));
export class commands { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(commands, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : commands), apply: () => commands, construct: () => new commands() }));
export class env { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(env, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : env), apply: () => env, construct: () => new env() }));
export class executeDocumentSymbolProvider { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(executeDocumentSymbolProvider, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : executeDocumentSymbolProvider), apply: () => executeDocumentSymbolProvider, construct: () => new executeDocumentSymbolProvider() }));
export class languages { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(languages, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : languages), apply: () => languages, construct: () => new languages() }));
export class open { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(open, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : open), apply: () => open, construct: () => new open() }));
export class packages { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(packages, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : packages), apply: () => packages, construct: () => new packages() }));
export class window { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(window, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : window), apply: () => window, construct: () => new window() }));
export class workspace { constructor(...args) { this.args = args; } static {} }
Object.setPrototypeOf(workspace, new Proxy(function () {}, { get: (t, k) => (k in t ? t[k] : workspace), apply: () => workspace, construct: () => new workspace() }));
