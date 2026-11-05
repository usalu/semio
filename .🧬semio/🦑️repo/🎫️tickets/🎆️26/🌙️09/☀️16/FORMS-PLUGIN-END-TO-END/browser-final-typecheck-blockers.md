# Actual Renderer Typecheck Diagnostics

Declared owner @semio-tech/framework-renderer-react:typecheck executed and failed after53.7seconds. Six diagnostics follow; generated sheet/touch/unnamedControl errors are resolved. No full typecheck pass is claimed.

```text
../../../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document/🟦️.ts(49,58): error TS2345: Argument of type 'PluginWasmHandle' is not assignable to parameter of type 'Readonly<{ readAppDocumentArchive?: ((instanceId: number) => Promise<DocumentArchivePack>) | undefined; readAppDocumentIdentity?: ((instanceId: number) => Promise<...>) | undefined; }>'.
  The types returned by 'readAppDocumentIdentity(...)' are incompatible between these types.
    Type 'Promise<AppDocumentIdentity>' is not assignable to type 'Promise<Readonly<{ parentDocumentId?: string | null | undefined; }>>'.
      Type 'AppDocumentIdentity' has no properties in common with type 'Readonly<{ parentDocumentId?: string | null | undefined; }>'.
../../../../🧱️elements/🏛️ShellHost/🟦️.tsx(190,438): error TS2339: Property 'currentSavedState' does not exist on type '{ closePrompt: Readonly<{ runtimeKey: string; name: string; phase: "confirm" | "saving"; failure: Readonly<{ code: string; }> | null; onKeep(): void; onDiscard(): void; }> | null; ... 114 more ...; onAction: (requested: Readonly<...>, submittedOrigin?: Readonly<...> | undefined, propagateFailure?: boolean, lifecycle...'.
../../../../🧱️elements/🖼️IconRenderHost/🧪️export/🟦️.ts(28,31): error TS2769: No overload matches this call.
  Overload 1 of 2, '(expected: readonly IconExportEvent[]): void', gave the following error.
    Argument of type '({ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; } | { kind: string; job: number; completed: number; owner?: undefined; label?: undefined; total?: undefined; atMs?: undefined; outcome?: undefined; } | { ...; } | { ...; })[]' is not assignable to parameter of type 'readonly IconExportEvent[]'.
      Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; } | { kind: string; job: number; completed: number; owner?: undefined; label?: undefined; total?: undefined; atMs?: undefined; outcome?: undefined; } | { ...; } | { ...; }' is not assignable to type 'IconExportEvent'.
        Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; }' is not assignable to type 'IconExportEvent'.
          Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; }' is not assignable to type 'Readonly<{ kind: "started"; job: number; owner: string; label: string; total: number; atMs: number; }>'.
            Types of property 'kind' are incompatible.
              Type 'string' is not assignable to type '"started"'.
  Overload 2 of 2, '(expected: NoInfer<readonly IconExportEvent[]>): void', gave the following error.
    Argument of type '({ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; } | { kind: string; job: number; completed: number; owner?: undefined; label?: undefined; total?: undefined; atMs?: undefined; outcome?: undefined; } | { ...; } | { ...; })[]' is not assignable to parameter of type 'readonly IconExportEvent[]'.
      Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; } | { kind: string; job: number; completed: number; owner?: undefined; label?: undefined; total?: undefined; atMs?: undefined; outcome?: undefined; } | { ...; } | { ...; }' is not assignable to type 'IconExportEvent'.
        Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; }' is not assignable to type 'IconExportEvent'.
          Type '{ kind: string; job: number; owner: string; label: string; total: number; atMs: number; completed?: undefined; outcome?: undefined; }' is not assignable to type 'Readonly<{ kind: "started"; job: number; owner: string; label: string; total: number; atMs: number; }>'.
            Types of property 'kind' are incompatible.
              Type 'string' is not assignable to type '"started"'.
../../../../🧱️elements/🗣️Interpreter/✍️input-views/🟦️.tsx(363,168): error TS2322: Type 'string | null' is not assignable to type 'string | undefined'.
  Type 'null' is not assignable to type 'string | undefined'.
../../../../🧱️elements/🗣️Interpreter/✍️input-views/🟦️.tsx(364,11): error TS2322: Type '{ id: string; "data-ui-node-id": number; "data-ui-node-key": string; pressed: boolean; text: string | undefined; icon: ReactElement<unknown, string | JSXElementConstructor<any>>; disabled: boolean; "aria-label": string | null; onPressedChange: (pressed: boolean) => UiControlCompletion; }' is not assignable to type 'IntrinsicAttributes & ToggleProps<string>'.
  Type '{ id: string; "data-ui-node-id": number; "data-ui-node-key": string; pressed: boolean; text: string | undefined; icon: ReactElement<unknown, string | JSXElementConstructor<any>>; disabled: boolean; "aria-label": string | null; onPressedChange: (pressed: boolean) => UiControlCompletion; }' is missing the following properties from type 'ToggleWithActionProps': kind, actionIcon, onActionClick
../../../../🧱️elements/🗣️Interpreter/🧱️node-tree/🟦️.tsx(204,61): error TS2322: Type 'ReactNode' is not assignable to type 'string'.
  Type 'undefined' is not assignable to type 'string'.
```


## Renderer Liveness Type Guard Correction

Declared renderer owner21435 reached actual tsc execution and terminated1 after52.3seconds. Its sole diagnostic was TS2367 at TextEditorHost.tsx line498: TypeScript retained the pre-await narrowing of the readonly owner.active getter and treated the second false comparison as impossible. Owner retirement can occur during the awaited action. The correction uses a fresh boolean helper read at both boundaries, preserving both runtime retirement checks without a cast, non-null assertion or relaxed guard.

A single fresh declared @semio-tech/framework-renderer-react:typecheck owner19028 now captures its result in generated/renderer-check-owner-liveness.log;21435 was confirmed terminal before launch. No typecheck pass is claimed yet. Browser8166 is retained, and completed focused69935/source-demand25541 remain green. This is a typechecking correction with equivalent liveness behavior; no new feature or fixture changed.
