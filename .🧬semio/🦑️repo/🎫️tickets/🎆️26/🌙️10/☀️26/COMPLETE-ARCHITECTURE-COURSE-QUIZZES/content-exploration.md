# Content Exploration

## Current Content

- Course: 🎓️teaching/🏛️architecture/📚️course/🔣️.json.
- Five quiz documents: 🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🌬️ventilation,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json.
- Topics already include energy, parametric, grasshopper, excel, bim, optimization in that order. The five new topics have empty quiz memberships and null unlocks.
- Contract: 🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json, Quiz definition; semio.quiz/v1.
- No nested AGENTS.md found under teaching or framework course product. Root instructions apply.

## Exact Authored Material Constraints

- Quiz ids cannot collide with topic ids or reserved root navigation ids; use leaf ids such as bim-information, parametric-dependencies, excel-references, grasshopper-data, optimization-objectives.
- Every visible text is bilingual en/de. Required Quiz properties: schema, id, emoji, title, description, tasks.
- Existing kinds are classification, sorting, matching. No new runtime implementation required for basic content.
- Each task must have a distinct task emoji in its quiz. Every item, category and matching dimension needs an icon with a supported motion; item emoji/motion pairs must be distinct within task.
- Every classification needs category descriptions or axes; profiles need at least three axes.
- Labels exceeding 40 Unicode characters need a short field. Short forms must be at most 40 characters, plain, unique per task and noun phrases suitable after N times. Avoid leading articles, commas, straight apostrophes and parenthetical details.
- Sorting/matching tasks need at least one familiar:true item. Sorting values must be distinct. Logarithmic values must be positive.
- Every task needs pool.slots and draw equal to slot count, at least two slots. Each slot needs at least five alternatives to pass five fresh attempts without repetition; all task items belong to exactly one slot. pool.seconds must be 300 for current course tests.
- Give every item bilingual explanation with reasoning, relevant value and source.

## Central Integration Points

- 🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts explicitly lazy-imports every quiz in course order; update imports.
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/📚️course/🔣️.json asserts exact quiz ids/order.
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🗂️topics/🔣️.json duplicates topics and memberships; maintain rejection vector assumptions.
- 🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🌬️ventilation/🔣️.json order currently covers entire energy course, and tests assert entire quiz list equals it. Update test to restrict energy membership or expand fixture deliberately.
- 🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️course/🟦️.ts validates content with owned TS core and independent AJV, balanced pools, icons, names, material order and topics. Existing test names mention placeholders; update wording.
- Course introduction only describes energy; generalize it. Add perfect-quiz badges and topic memberships, choose unlocked status explicitly.
- Teaching README requires new semantic directories registered in repository taxonomy. No taxonomy changes made during exploration.

## Validation

Run repo targets @teaching/architecture-course:test and @teaching/architecture-course:check. The first uses TypeScript domain validators and AJV draft-07 oracle, the latter Rust proctor validation. Read package/target configuration before selecting narrower test filters.

## Recommended Ownership

Keep course, fixtures, lazy import module and taxonomy integration with one owner. Content writers can own disjoint new topic trees. Existing quiz contract supports all five domains without schema changes.

## Tool Notes

Use repo for commands. rg is not in tool registry; repo tools exec rg fails. Python requires -X utf8 on native Windows for emoji paths. Prune node_modules, caches and build outputs during filesystem walks.
