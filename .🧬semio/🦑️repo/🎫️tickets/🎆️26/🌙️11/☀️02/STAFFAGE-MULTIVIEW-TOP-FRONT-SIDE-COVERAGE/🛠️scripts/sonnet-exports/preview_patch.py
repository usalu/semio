import re
import sys

p = sys.argv[1]
raw = open(p, encoding="utf-8", newline="").read()
crlf = "\r\n" in raw
s = raw.replace("\r\n", "\n")


def rep(a, b):
    global s
    assert a in s, a[:70]
    s = s.replace(a, b, 1)


a = s.index("/** 🗒️ One sheet of the side, front and top view")
b = s.index("function sheet(rows")
s = s[:a] + s[b:]
rep("import { solveStructure }", 'import { PROJECTIONS, resolveSheet, type Projection } from "../../📤️export/📑️sheet/🟦️.ts";\nimport { solveStructure }')
rep("      const rep = asset.representations[`${denominator}` as keyof Asset[\"representations\"]];\n", "")
rep("      const captions = VIEWPOINTS.map((at) => viewpointCaption(at, sectionOf(rep?.status === \"authored\" ? rep : undefined, at), language));\n", "")
old = s[s.index('      for (const view of flags.has("view") ? views : (["dark-line"] as const)) emit('):]
old = old[: old.index("\n") + 1]
new = '''      const projection = flags.get("projection") as Projection | undefined;
      if (projection === undefined || !(PROJECTIONS as readonly string[]).includes(projection)) throw new Error(`--views needs --projection ${PROJECTIONS.join("|")}\\n${USAGE}`);
      const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...drawOptions } = options("dark-line", true, "elevation");
      const source = selector.kind === "symbol" ? ({ kind: "symbol", symbol: selector.symbol, denominator } as const) : ({ kind: "scale", denominator } as const);
      for (const view of flags.has("view") ? views : (["dark-line"] as const)) emit(`${asset.id}--${when}--views-${view}-${projection}-${tag}`, writeSvg(resolveSheet(asset, query, source, view, drawOptions as never, projection, language).sheet));
'''
s = s.replace(old, new, 1)
s = s.replace("[--views] [--sheet]", "[--views --projection first|third] [--sheet]")
s = s.replace("(--viewpoint elevation|front|plan | --views)", "(--viewpoint elevation|front|plan | --views --projection first|third)")
s = s.replace("`--views` writes one sheet per view with the side, front and top view of the same pose or clip instant side by side at the same scale on a common ground line.", "`--views --projection first|third` writes one sheet per view in the D7 arrangement of `📤️export/📑️sheet` (plan above or below the side elevation, front beside it) with the three views of the same pose or clip instant at the same scale.")
if crlf:
    s = s.replace("\n", "\r\n")
open(p, "w", encoding="utf-8", newline="").write(s)
