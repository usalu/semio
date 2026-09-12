# Command Router Argument Adversaries

## Corrected Public Classifier Probe

After the executor reported completion, root reran the exact retained ten inputs through the public classifier. The one delegation positive remains `tool-metadata`; all nine hostile controls now return `unresolved`, matching every expected result. The before/after rows are saved under generated/coordinator/router-argument-adversaries-corrected.json. This confirms the exact reported correction; independent Terra review of fresh variants and the broader grammar is now active. It does not close the separate non-package enforcement gap.

## Independent Oracle Review

Root also read the installed-TypeScript AST oracle in the package-boundary-classification test. At this snapshot, `terminal()` and `router()` recognize syntax without recursively checking arguments; property/element handling in `literals()` ignores computed indices and has a broad process/import.meta/this prefix fallback; class method parameter initializers/decorators are not checked; and for-of permits any admitted local name without proving finite literal provenance. These are concrete code-review observations, distinct from the executed ten-vector classifier probe below. The executor was asked to add fresh controls for these seams, template interpolation, computed object keys, destructuring defaults and alias shadowing, while retaining an independently implemented AST oracle.

Observed 2026-09-12 15:22 UTC against the current public classifyPackageSourceDisposition API, root-script disposition and TypeScript grammar. All ten inputs parse with the installed TypeScript compiler with zero syntax diagnostics. The positive delegation is accepted; all nine hostile controls are also incorrectly accepted as tool-metadata. These results concern this implementation snapshot and do not attribute causality to any author.

The nine false admissions show that imported outer callees do not prove the provenance or purity of arguments, member-call receivers, initializer entries, conditions, finite iteration, router registrations or terminal arguments. A closed recursively checked grammar needs binding provenance and statement context, not additional effect-name blacklists. Keep legitimate delegation and pair each allowed grammar expansion with a hostile control. Imported operations may be delegated, but arbitrary member invocations and unproven iterables are not thin command routing.

## delegate

Expected tool-metadata; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {await execute(args);} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## argument-member-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {await execute(globalThis.sideEffect());} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## console-member-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {console.log(globalThis.sideEffect()); await execute(args);} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## array-member-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {const jobs = [globalThis.sideEffect()]; await execute(jobs);} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## object-member-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {const value = { payload: globalThis.sideEffect() }; await execute(value);} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## condition-member-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {if (globalThis.sideEffect()) { await execute(args); }} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```

## unbounded-imported-iterable

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";
import { jobs } from "./owner/🟦️.ts";
class Build extends BundleScript { async run(args: string[]) { for (const job of jobs) { await execute(job); } } }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);```

## terminal-argument-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";
await runBundleScriptMain(import.meta, globalThis.sideEffect());```

## registration-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {await execute(args);} }
const router = new ScriptRouter([Build, globalThis.sideEffect()]);
await runBundleScriptMain(import.meta, router);
```

## closure-argument-effect

Expected unresolved; observed tool-metadata; native syntax diagnostics 0.

```ts
import { BundleScript, ScriptRouter, runBundleScriptMain } from "./📜️script.ts";
import { execute } from "./owner/🟦️.ts";

class Build extends BundleScript { async run(args: string[]) {const invoke = () => execute(globalThis.sideEffect()); await invoke();} }
const router = new ScriptRouter([Build]);
await runBundleScriptMain(import.meta, router);
```
