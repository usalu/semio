# Preview Chapter Title Audit

Read-only inspection; no media join or source mutation.

The canonical 26-entry curriculum manifest has only id, script, chapters and output per video. None exposes an explicit localized EN/DE lesson title; its schema requires those four fields and disallows extra properties. The raw output labels are not a complete localized title source. Adding titles to that canonical manifest/schema would change frozen curriculum inputs during v9 and require revalidation/freshness checks; do not do that while the run is active.

The active Nov 1 course_preview.py writes each raw lesson id as the MP4 chapter title. verify-preview.py currently asserts chapter titles equal lesson ids; course-preview-contract.json stores only ordered IDs, and preview-freeze.py verifies receipt IDs/order and media identities rather than chapter display titles. For people navigating a preview, identifiers such as grasshopper_anatomy are less clear than localized, human-readable labels. The supplied titleized-title rule explicitly concerns ticket/goal titles; applying it to chapter labels is a product-quality interpretation, not a separate explicit requirement. English labels should use natural title case and German labels natural German capitalization, rather than mechanically title-casing German.

Minimal schema-first plan after v9 and coordinator authorization: define an explicit complete 26-by-2 title mapping in a retained preview title contract with a language-neutral schema requiring exactly EN and DE nonempty titles and each curriculum ID once. Author and review the titles, rather than deriving German from IDs. Update the joiner to resolve a title for chapter metadata by language while keeping canonical id for ordering, source lookup and receipt lesson id; fail closed for missing/duplicate IDs or translations. Update verify-preview.py to compare actual MP4 chapter titles with the same selected localized mapping while preserving its separate receipt-ID/order assertion. The receipt may retain IDs alone or add title separately; never replace IDs with translated strings. Keep the post-decode freeze gate ID and SHA/media semantics unchanged unless its contract itself is intentionally revised.

Tests: validate language-neutral mapping with independent JSON Schema implementation; assert 26 IDs match curriculum order and bilingual coverage; test title lookup including missing/duplicate and special Unicode/FFmetadata escaping; create a small two-chapter synthetic audio/video via registered ffmpeg, then read chapter titles through PyAV and check localized display titles and unchanged IDs/order in receipt. Run EN and DE examples and ensure invalid mappings fail closed. This is a proposed change only; exact title wording needs authored review and no existing title translation is established by the manifest.

## Implemented localized chapters

The canonical presentation hierarchy, unlike the curriculum manifest, provides one explicit EN/DE lesson title for each of all 26 IDs. Retained Nov 1 helper chapter_titles.py resolves that hierarchy, validates unique IDs and both nonblank locale values, and escapes FFmetadata. course_preview.py now writes localized chapter titles, rechecks titles after join and records each title alongside its unchanged lesson ID; verify-preview.py checks both receipt and MP4 chapter titles. preview-freeze.py rechecks each receipt title against the current localized resolver after decode. No native source, curriculum schema or v9 registry was changed; actual join remains pending.

Under individual normal 1 GiB grants, chapter-title-test.py passed 3/3: schema validation for all 26 catalog entries, missing/duplicate/invalid locale negatives and actual two-chapter EN/DE Unicode/escaped FFmetadata MP4 round trips through registered FFmpeg and PyAV. preview-test.py passed 2/2 actual join/audio/ordering and publication-race tests. preview-freeze-run-tests.py passed 3/3 including changed receipt and resolver titles. Generated synthetic media remain disposable. All 24 monitored v4/source/registry hashes matched after changes.

Retained changes: Nov 1 🧰️r3-media/chapter_titles.py (new) and course_preview.py; current ticket chapter-title.schema.json, chapter-title-test.py, preview-test.py, verify-preview.py, preview-freeze.py and preview-freeze-test.py. The original audit assertion that no localized titles exist is superseded: it referred only to the curriculum manifest, while the presentation hierarchy is the authoritative existing source.

## Resolved flat chapter labels

The flat MP4 list requires topic context: several otherwise unique labels such as Cell References omit their Excel topic. Each label combines existing localized topic and lesson titles unless the topic already occurs in the lesson title. The resolver rejects duplicate flat labels. Current resolved order:

| ID | English | German |
|---|---|---|
| physical_fundamentals | Energy · Physical Fundamentals | Energie · Physikalische Grundlagen |
| ventilation | Energy · Ventilation | Energie · Lüftung |
| heating | Energy · Heating | Energie · Heizen |
| cooling | Energy · Cooling | Energie · Kühlen |
| energy_balance | Energy Balance | Energiebilanz |
| bim_information | Building Information Modelling · Data and Information | Building Information Modelling · Daten und Information |
| bim_objects | Building Information Modelling · Object Orientation | Building Information Modelling · Objektorientierung |
| bim_representations | Building Information Modelling · Object Representations | Building Information Modelling · Repräsentationen von Objekten |
| bim_models | Building Information Modelling · Partial Models and Translation | Building Information Modelling · Teilmodelle und Modellübersetzungen |
| bim_exchange | Building Information Modelling · Interfaces and Model Translation | Building Information Modelling · Schnittstellen und Modellübersetzung |
| bim_consistency | Building Information Modelling · Consistency across Models | Building Information Modelling · Konsistenz über Modelle hinweg |
| grasshopper_anatomy | Anatomy of a Grasshopper Script | Aufbau eines Grasshopper-Skripts |
| grasshopper_capsule | Grasshopper · One Capsule, Parametric | Grasshopper · Eine Kapsel, parametrisch |
| grasshopper_tower | Grasshopper · From One Capsule to the Tower | Grasshopper · Von der Kapsel zum Turm |
| grasshopper_casting | Grasshopper · Casting: When Types Meet | Grasshopper · Casting: Wenn Typen sich treffen |
| grasshopper_trees | Grasshopper · Data Trees on the Tower | Grasshopper · Datenbäume am Turm |
| excel_anatomy | Excel · The Anatomy of a Workbook | Excel · Aufbau einer Arbeitsmappe |
| excel_references | Excel · Cell References | Excel · Zellbezüge |
| excel_program | The Room Program in Excel | Das Raumprogramm in Excel |
| excel_energy | Excel · Heating Demand in the Nakagin Workbook | Excel · Heizwärmebedarf in der Nakagin-Arbeitsmappe |
| optimization_extrema | Optimization · Minimization and Maximization | Optimierung · Minimierung und Maximierung |
| optimization_sampling | Optimization · Sampling the Design Space | Optimierung · Sampling des Entwurfsraums |
| optimization_experiments | Optimization · Design of Experiments | Optimierung · Design of Experiments |
| optimization_index | Optimization · Multiple Criteria and Indices | Optimierung · Multikriteriell und Index |
| optimization_pareto | Optimization · Pareto Fronts | Optimierung · Pareto-Fronten |
| optimization_algorithms | Optimization Algorithms | Optimierungsalgorithmen |

After this contextual-label correction, chapter-title-test.py passed 3/3, preview-test.py 2/2 and preview-freeze-run-tests.py 3/3 under normal 1 GiB grants. No course join or native input change.
