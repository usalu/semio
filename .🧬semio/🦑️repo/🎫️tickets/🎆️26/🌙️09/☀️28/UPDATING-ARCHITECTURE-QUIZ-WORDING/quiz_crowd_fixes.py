"""🗳️ Fixes found by the §17 suites: a result row nobody else answered shows a dash with the sentence "Nobody has answered
this yet." for assistive technology (the cell used to stay empty, since an element that renders nothing is not
nothing), and the place of a sorting rounds half up to one decimal (1.9999 is place 2) instead of the scores' "never
reads as whole" rounding. Exact anchors, each found once."""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")


def edit(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:90]!r}")
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{relative}: write failed")


edit("🏁️results/🟦️.tsx", [
    ("function Others(props: { readonly shown: boolean; readonly children: ReactNode }): ReactElement | null {\n"
     "  return props.shown ? <td className={EDGE}>{props.children ?? \"–\"}</td> : null;\n"
     "}\n",
     "function Others(props: { readonly crowd: ReadonlyMap<string, ItemCrowd> | undefined; readonly item: string; readonly text: QuizText; readonly children: (item: ItemCrowd) => ReactNode }): ReactElement | null {\n"
     "  const { crowd, text } = props;\n"
     "  if (crowd === undefined) return null;\n"
     "  const item = crowd.get(props.item);\n"
     "  return (\n"
     "    <td className={EDGE}>\n"
     "      {item === undefined ? (\n"
     "        <>\n"
     "          <span aria-hidden=\"true\">–</span>\n"
     "          <span className=\"sr-only\">{text(\"quiz.crowd.nobody\")}</span>\n"
     "        </>\n"
     "      ) : (\n"
     "        props.children(item)\n"
     "      )}\n"
     "    </td>\n"
     "  );\n"
     "}\n"),
    ("          <Others shown={crowd !== undefined}>\n"
     "            <CrowdChoices item={crowd?.get(item.item)} subject={labelOf(task.items, item.item, locale)} label={(key) => labelOf(task.categories, key, locale)} text={text} />\n"
     "          </Others>\n",
     "          <Others crowd={crowd} item={item.item} text={text}>\n"
     "            {(others) => <CrowdChoices item={others} subject={labelOf(task.items, item.item, locale)} label={(key) => labelOf(task.categories, key, locale)} text={text} />}\n"
     "          </Others>\n"),
    ("            <Others shown={crowd !== undefined}>\n"
     "              <CrowdPosition item={crowd?.get(item.item)} total={result.items.length} subject={labelOf(task.items, item.item, locale)} text={text} locale={locale} />\n"
     "            </Others>\n",
     "            <Others crowd={crowd} item={item.item} text={text}>\n"
     "              {(others) => <CrowdPosition item={others} total={result.items.length} subject={labelOf(task.items, item.item, locale)} text={text} locale={locale} />}\n"
     "            </Others>\n"),
    ("                  <Others shown={others !== undefined}>\n"
     "                    <CrowdChoices item={others?.get(item.item)} subject={labelOf(task.items, item.item, locale)} label={(key) => value(Number(key))} text={text} />\n"
     "                  </Others>\n",
     "                  <Others crowd={others} item={item.item} text={text}>\n"
     "                    {(entry) => <CrowdChoices item={entry} subject={labelOf(task.items, item.item, locale)} label={(key) => value(Number(key))} text={text} />}\n"
     "                  </Others>\n"),
])
edit("🗳️crowd/🟦️.tsx", [
    ("/** 📍️ The place (1 … `total`) a normalized position stands for, to one decimal. */\n"
     "export function crowdPlace(position: number, total: number): number {\n"
     "  return oneDecimal(1 + position * Math.max(0, total - 1));\n"
     "}\n",
     "/** 📍️ The place (1 … `total`) a normalized position stands for, rounded half up to one decimal. */\n"
     "export function crowdPlace(position: number, total: number): number {\n"
     "  return Math.round((1 + position * Math.max(0, total - 1)) * 10) / 10;\n"
     "}\n"),
    ('import { formatNumber, oneDecimal } from "../📏️quantity/🟦️.ts";\n', 'import { formatNumber } from "../📏️quantity/🟦️.ts";\n'),
])
print("[crowd fixes] done")
