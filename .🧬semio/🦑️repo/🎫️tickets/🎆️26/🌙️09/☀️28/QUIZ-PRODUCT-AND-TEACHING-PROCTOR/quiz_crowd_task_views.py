"""🗳️ Puts what the others think into the task views (design §17) and makes items and categories presence anchors:
classification items show the others' categories, sortings the others' places, matching rows who matched (live) or the
others' values (submitted); every item carries `data-quiz-item` (the drag reports it) and `item:<id>`, bins
`category:<id>`. Exact anchors, each found once per file."""

import pathlib
import sys
import time

MODULES = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules")


def edit(relative: str, edits: list[tuple[str, str]]) -> None:
    path = MODULES / relative
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            sys.exit(f"{relative}: anchor found {text.count(old)} times: {old[:80]!r}")
        text = text.replace(old, new)
    for attempt in range(40):
        try:
            path.write_text(text, encoding="utf-8", newline="")
            return
        except OSError:
            time.sleep(0.5)
    sys.exit(f"{relative}: write failed")


edit("👥️presence/🟦️.tsx", [
    ("  quiz: (quiz: string): Anchor => `quiz:${quiz}`,\n",
     "  quiz: (quiz: string): Anchor => `quiz:${quiz}`,\n  item: (item: string): Anchor => `item:${item}`,\n  category: (category: string): Anchor => `category:${category}`,\n"),
])
edit("🗂️classification/🟦️.tsx", [
    ('import { DROP_ZONE_CLASS, DragGrip, LiveRegion, SELECT_CLASS, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n',
     'import { DROP_ZONE_CLASS, DragGrip, LiveRegion, SELECT_CLASS, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n'
     'import { CrowdChoices, useCrowd } from "../🗳️crowd/🟦️.tsx";\n'
     'import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
    (" * diagram over the task's axes.\n */",
     " * diagram over the task's axes. Each item shows where the others put it; items and bins are presence anchors, so the\n"
     " * others' pointers land on the same item wherever it sits on each sheet.\n */"),
    ("  const axes: readonly RadarAxis[] =", "  const crowd = useCrowd().crowd?.items(task);\n  const axes: readonly RadarAxis[] ="),
    ('    <li key={item.id} data-quiz-drag="" className=',
     '    <li key={item.id} data-quiz-drag="" data-quiz-item={item.id} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)} className='),
    ("        ))}\n      </select>\n    </li>",
     "        ))}\n      </select>\n"
     '      <CrowdChoices item={crowd?.get(item.id)} subject={itemLabel(item)} label={categoryLabel} text={text} className="col-start-2" />\n'
     "    </li>"),
    ("data-quiz-drop={`${CATEGORY_ZONE}${category.id}`} aria-labelledby",
     "data-quiz-drop={`${CATEGORY_ZONE}${category.id}`} data-presence-anchor={PRESENCE_ANCHORS.category(category.id)} aria-labelledby"),
])
edit("↕️sorting/🟦️.tsx", [
    ('import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n',
     'import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, elementId, useAnnouncement, useFocusAfterRender, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n'
     'import { CrowdPosition, useCrowd } from "../🗳️crowd/🟦️.tsx";\n'
     'import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
    (" * counts as an answer once the learner moves an item or keeps the order explicitly.\n */",
     " * counts as an answer once the learner moves an item or keeps the order explicitly. Each item shows where the others\n"
     " * place it, and is a presence anchor.\n */"),
    ("  const quantity = localized(task.quantity.label, locale);\n",
     "  const quantity = localized(task.quantity.label, locale);\n  const crowd = useCrowd().crowd?.items(task);\n"),
    ('data-quiz-drag="" data-quiz-drop={`${ITEM_ZONE}${id}`}>',
     'data-quiz-drag="" data-quiz-item={id} data-presence-anchor={PRESENCE_ANCHORS.item(id)} data-quiz-drop={`${ITEM_ZONE}${id}`}>'),
    ("              ↓\n            </button>\n          </li>",
     "              ↓\n            </button>\n"
     '            <CrowdPosition item={crowd?.get(id)} total={order.length} subject={label(id)} text={text} locale={locale} className="col-span-3 col-start-3" />\n'
     "          </li>"),
])
edit("🃏️matching/🟦️.tsx", [
    ('import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, SELECT_CLASS, elementId, useAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n',
     'import { DragGrip, ICON_BUTTON_CLASS, LiveRegion, SELECT_CLASS, elementId, useAnnouncement, type TaskViewProps } from "../🧩️task/🟦️.tsx";\n'
     'import { CrowdChoices, useCrowd } from "../🗳️crowd/🟦️.tsx";\n'
     'import { PRESENCE_ANCHORS } from "../👥️presence/🟦️.tsx";\n'),
    (" * pointer users drag a card by its grip onto an item's row.\n */",
     " * pointer users drag a card by its grip onto an item's row. Each item row shows which values the others give it, and\n"
     " * is a presence anchor.\n */"),
    ("  const cardText = (dimension: SheetDimension, index: number): string => formatQuantity(dimension.cards[index] ?? 0, dimension.quantity, locale);\n",
     "  const cardText = (dimension: SheetDimension, index: number): string => formatQuantity(dimension.cards[index] ?? 0, dimension.quantity, locale);\n"
     "  const { crowd } = useCrowd();\n"),
    ("        const quantity = localized(dimension.quantity.label, locale);\n        const drop =",
     "        const quantity = localized(dimension.quantity.label, locale);\n        const others = crowd?.items(task, dimension.id);\n        const drop ="),
    ("                    <tr key={item.id} data-quiz-drop={`${SLOT_ZONE}${dimension.id}:${item.id}`}>\n"
     "                      <th scope=\"row\" className={ROW_HEAD}>\n"
     "                        {itemLabel(item.id)}\n"
     "                      </th>",
     "                    <tr key={item.id} data-quiz-drop={`${SLOT_ZONE}${dimension.id}:${item.id}`} data-presence-anchor={PRESENCE_ANCHORS.item(item.id)}>\n"
     "                      <th scope=\"row\" className={ROW_HEAD}>\n"
     "                        {itemLabel(item.id)}\n"
     "                        <CrowdChoices item={others?.get(item.id)} subject={itemLabel(item.id)} label={(key) => formatQuantity(Number(key), dimension.quantity, locale)} text={text} className=\"font-normal\" />\n"
     "                      </th>"),
])
print("[crowd task views] done")
