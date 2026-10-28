"""Batch `rootguard`: cache partitions are only opened for the checkout root. Producer of the second image set `🧰️framework/.cache/b` (8.1 GiB, tools image 8.0 GiB, written as late as 10-26 18:59): a process whose working directory is `🧰️framework` (Nx targets with `cwd: 🧰️framework`, the command runner whose Cargo workspace is `🧰️framework`) passed `process.cwd()` as the workspace and the partition opener created `.cache/b/<partition>` below it.
The opener (`openRepositoryCachePartitionAttemptV1`; the pure path resolvers `repositoryCacheBudgetV1` and `📂️paths` stay unguarded, `repo clean` protection and path tests compute them for any root) now refuses a directory that lies inside a checkout without being its root (EN + DE)."""
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
S = f"{L}/⚡️caching/💾️storage"
B = f"{S}/🔒️enforcement/📦️budget"
TEXTS = f"{S}/🧾️ledger/📖️texts/🔣️.json"

EDITS = [
    (TEXTS, [(
        '    "error.undeclared": { "en": "Declared cache area required: {segment} (declare it in the cache ledger)",',
        '    "workspace.nested": { "en": "Cache storage belongs to the checkout root {root}, not to {workspace} inside it (a second set of images would be created there); run the command from the checkout root", "de": "Cache-Speicher gehört zur Checkout-Wurzel {root}, nicht zu {workspace} darin (dort würde ein zweiter Satz Images entstehen); starte den Befehl in der Checkout-Wurzel" },\n'
        '    "error.undeclared": { "en": "Declared cache area required: {segment} (declare it in the cache ledger)",',
    )]),
    (f"{B}/📥️opening/🟦️.ts", [
        ('import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../🟦️.ts";',
         'import {repositoryCacheBudgetV1,type RepositoryCachePartitionIdV1} from "../🟦️.ts";\nimport {requireRepositoryWorkspaceRootV1} from "../🧭️root/🟦️.ts";'),
        (' checkResourceLeaseControl(control);const budget=repositoryCacheBudgetV1(workspace),declaration=budget.partitions[id];',
         ' checkResourceLeaseControl(control);requireRepositoryWorkspaceRootV1(workspace);const budget=repositoryCacheBudgetV1(workspace),declaration=budget.partitions[id];'),
    ]),
]

NEW = [
    (f"{B}/🧭️root/🟦️.ts", "root.ts"),
    (f"{B}/🧭️root/🧪️tests/🟦️.ts", "root-test.ts"),
]
