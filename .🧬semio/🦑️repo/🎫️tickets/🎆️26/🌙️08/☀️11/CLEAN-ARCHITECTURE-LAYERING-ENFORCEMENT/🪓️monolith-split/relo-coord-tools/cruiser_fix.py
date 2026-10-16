"""🩹️ Removes the cross-owner relative imports slice A's TS twins introduced: tool-run wire aliases back to the manifest twin, `WindowLayout` re-exported by the manifest twin, the history-edit test through the `@semio-tech/framework` export target.

Usage: python3 cruiser_fix.py
"""
import re

R = "/Users/ueli/Documents/semio/🧰️framework/"
MANIFEST, KERNEL, OSCONFIG = R + "🔨️modules/🛂️manifest/🟦️.ts", R + "🔨️modules/🎠️kernel/🟦️.ts", R + "🛍️products/💻️os/🎚️config/🧬️schema/🟦️.ts"
BARREL, TOOLRUN, TTTEST = R + "📦️packages/🟦️typescript/🟦️.ts", R + "🔨️modules/⏯️tool-run/🕹️actions/🟦️.ts", R + "🔨️modules/⏪️time-travel/🕹️actions/🧪️tests/🟦️.ts"


def edit(path, old, new):
    text = open(path, encoding="utf-8").read()
    assert text.count(old) == 1, (path, old[:80], text.count(old))
    open(path, "w", encoding="utf-8").write(text.replace(old, new))


tool = open(TOOLRUN, encoding="utf-8").read()
gen_import = re.search(r'^import type \{[^}]*\} from "\.\./\.\./🛂️manifest/🤖️generated/🪪️manifest/🟦️\.ts";\n', tool, re.M).group(0)
alias_block = re.search(r"^/\*\* ⏯️ Generated from Rust `ToolRunDefinition`.*?^export type JobKindId = GeneratedJobKindId;\n", tool, re.M | re.S).group(0)
names = re.search(r"import type \{([^}]*)\}", gen_import).group(1).strip()
edit(TOOLRUN, gen_import, "")
edit(TOOLRUN, alias_block, "")
edit(TOOLRUN, "readonly tools: readonly { readonly run?: ToolRunDefinition }[]; readonly utilities: readonly { readonly run?: ToolRunDefinition }[]", "readonly tools: readonly { readonly run?: unknown }[]; readonly utilities: readonly { readonly run?: unknown }[]")

edit(MANIFEST, 'import type { WindowLayout } from "./🗺️window-layout/🟦️.ts";\n', 'import type { WindowLayout } from "./🗺️window-layout/🟦️.ts";\nexport type { WindowStackCorner, WindowLayoutWindowNode, WindowLayoutStackNode, WindowLayoutAxisNode, WindowLayout } from "./🗺️window-layout/🟦️.ts";\nimport type { ' + names + ' } from "./🤖️generated/🪪️manifest/🟦️.ts";\n')
manifest = open(MANIFEST, encoding="utf-8").read()
anchor = "export type ToolDefinition = GeneratedToolDefinition;\n"
assert manifest.count(anchor) == 1
edit(MANIFEST, anchor, anchor + "\n" + alias_block)

edit(KERNEL, '  NamedLayout,\n} from "../🛂️manifest/🟦️.ts";\nimport type { WindowLayout } from "../🛂️manifest/🗺️window-layout/🟦️.ts";\n', '  NamedLayout,\n  WindowLayout,\n} from "../🛂️manifest/🟦️.ts";\n')
edit(OSCONFIG, 'import type { WindowLayout } from "../../../../🔨️modules/🛂️manifest/🗺️window-layout/🟦️.ts";', 'import type { WindowLayout } from "../../../../🔨️modules/🛂️manifest/🟦️.ts";')
edit(BARREL, 'export * from "../../🔨️modules/🛂️manifest/🗺️window-layout/🟦️.ts";\n', "")
edit(TTTEST, 'import { DIALOG_CHOICE_ARG } from "../../../🛂️manifest/🟦️.ts";', 'import { DIALOG_CHOICE_ARG } from "@semio-tech/framework";')
print("ok", names[:120])
