# 🦗 Grasshopper research (Rhino 8, Grasshopper 1, bilingual EN/DE)

Scope: vocabulary, casting, data trees and list matching for the teaching clips. Quiz sources read in full: `🎓️teaching/🏛️architecture/🦗️grasshopper/❓️quiz/🔣️.json` (tasks: canvas-terminology 45 items, type-casting 10, tree-operations 15, list-matching 15). Parametric quiz and assignment were read for overlap (parameter domains, slider semantics).

Verification status: `[V]` = confirmed against a fetched McNeel source; `[K]` = from working knowledge of Grasshopper 1, not re-checked in this session; `[CHECK]` = verify in the running Rhino 8 UI before filming.

---

## 1. Anatomy

| Term (EN) | Term (DE, proposed) | Definition | Notes |
|---|---|---|---|
| Definition / script (.gh, .ghx) | Definition (Skript) | The saved Grasshopper graph. `.gh` is the binary format, `.ghx` the XML format. | `[V]` McNeel: Grasshopper definitions. Use "Definition" consistently; "Skript" for the general sense. |
| Canvas | Arbeitsfläche (Canvas) | Drawing area where components are placed and wired. | Keep the English word "Canvas" in DE; it is the UI term. |
| Component | Komponente | An operation that consumes inputs and produces outputs. | Quiz: "An operation consumes input data and produces computed output data." Shown as a box with nickname on top. |
| Nickname | Kurzname | Short label shown on the component (e.g. "Add"). | `[K]` set via right-click; the full name appears on hover/Component info. |
| Input (port on the left) | Eingang | Left-side port that receives data for its owner. | Quiz "input" category. |
| Output (port on the right) | Ausgang | Right-side port that supplies computed/stored data. | Quiz "output" category. |
| Grip | Griff | Small handle on a parameter/component for interaction (e.g. slider handle, panel grip). | `[K]` |
| Wire | Verbindung (Draht) | Link from an output to an input. | Quiz: "A wire alone does not construct missing geometry" (type-casting prompt). |
| Preview | Vorschau | Toggle that draws the output in the Rhino viewport. | `[K]` |
| Enabled | Aktiviert | Toggle that switches a component on/off for solving. | `[K]` |
| Parameter | Parameter | A typed container that stores or relays data. Quiz: "A typed container stores or relays data; the Params tab supplies generic containers." | Floating parameter (no owner) vs component parameter (belongs to a component's port). |
| Floating parameter | Freier Parameter | A parameter placed on canvas that is not attached to a component; it stores data (e.g. a Number capsule). | Quiz items "Number capsule", "Point capsule", etc. Standard DE term for capsule: "Kapsel" (quiz uses "Number-Kapsel"). |
| Component parameter | Komponentenparameter | The input/output port data definition belonging to a component. | `[K]` |
| Input type: Number Slider | Zahlenschieberegler (Slider) | Interactive numeric control with domain and rounding. Quiz: "An interactive numeric control supplies a value within a configured domain." | `[V]` quiz; `[CHECK]` the rounding options in Rhino 8 (see §1.1). |
| Input type: Panel | Panel | Text display that shows data in full. Quiz: "A text display inspects incoming data or supplies text when unconnected." | Shows paths in tree view (`{0;1}` style). Input Panel can also type text when unconnected. |
| Input type: Boolean Toggle | Boolescher Schalter (Boolean Toggle) | Two-state input (True/False). | `[K]` |
| Input type: Value List | Werteliste | Dropdown of fixed items. | `[K]` |
| Plugin | Plugin / Erweiterung | Installed extension package adding components. Quiz: "An installed extension package adds tools to Grasshopper." | Use "Plugin" in DE too (already used in quiz). |
| Package / .gha | Paket / Assembly (.gha) | Plugin binary (.gha) loaded from the plugin folder; installed via Package Manager. | `[K]` |
| Package Manager | Paketverwaltung | Built-in Grasshopper tool to install plugins. | `[CHECK]` menu location in Rhino 8. |
| Food4Rhino | Food4Rhino | Community download site for plugins. | `[K]` |
| Example plugins | Beispiel-Plugins | Ladybug (climate/energy), Karamba3D (structural), Kangaroo (physics/form-finding). | `[K]` Use as examples only; the quiz treats such tools as external dependencies. |
| Data | Daten | Anything passed between ports. | |
| Data item | Element (Item) | One value (number, point, curve, ...). | Quiz: "item". |
| List | Liste | Ordered sequence of items within one branch. | |
| Branch | Ast / Zweig | One list inside a tree, addressed by a path. | Quiz: "One path identifies one ordered list inside a data tree." Prefer "Ast" in DE clips, with "Zweig" as synonym (the quiz uses Zweig). |
| Tree | Datenbaum | The collection of branches with their paths. | Quiz: "The complete collection associates branch paths with ordered lists." |
| Path | Pfad | Address of a branch, e.g. `{0;1;2}`. | |
| Index | Index | Position of an item inside its list, starting at 0. | Quiz Panel item "Index display". |

### 1.1 Slider settings (Number Slider)
- A slider has a **domain** (lower and upper bound) and a **rounding** setting. `[CHECK]` The brief lists "R/N/E/O"; the Rhino 8 Grasshopper 1 slider menu shows rounding options such as Integer, Float, Even and Odd. Confirm the exact labels in the running app before writing captions.
- Quiz rule (parametric quiz, `parameter-domains`): a slider is continuous or discrete depending on its settings. An unrounded slider with domain 0 to 45 is continuous in the idealized model; an integer-rounded slider is discrete.

### 1.2 Component colour states (Grasshopper 1, `[V]` for orange and red semantics)
- **Grey**: solution succeeded. Normal state.
- **Orange**: warning. The component ran but something is off (e.g. data was truncated, a mismatch was tolerated).
- **Red**: error. The component failed; its output is blank or incomplete.
- Panels and sliders are not solved components in the same way; their state shows their own feedback.
- Teaching point: a red component does not mean the whole definition is broken; trace the first red component upstream. Downstream components turn grey only after upstream is fixed.

---

## 2. Casting (implicit type conversion)

### 2.1 Mechanism `[V]`
Source: McNeel Grasshopper data types guide (https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/).
1. If the source is already a more derived type of the target, the data passes through.
2. Otherwise Grasshopper calls `CastTo<target>` on the source.
3. If that fails, it creates an empty target and calls `CastFrom<source>` on it.
4. If both fail: "Grasshopper will put up an error and leave that slot blank."
5. A conversion may lose information ("the best I can do under these imperfect circumstances").

Example given by McNeel: a Boolean casts to integer (false→0, true→1), to colour, and to string ("false"/"true").

### 2.2 Casting table for the clips (teaching examples)

Each row: source → target parameter. Status is what I can support.

| Source → target | Result | Status | Teaching note |
|---|---|---|---|
| Integer 2 → Number | 2 (exact) | `[V]` quiz `cast-integer` | Widening is safe. |
| Text "12" → Number | 12 (number, not text) | `[V]` quiz `cast-text` | Only plain numeric text. Locale and decimal separators matter (`12,5` vs `12.5`). |
| Number 3 → Text | "3" | `[V]` quiz `cast-number-text` | Later numeric ops need a new numeric cast. |
| Point → Vector | (x,y,z) components | `[V]` quiz `cast-point-vector` | A direction and magnitude from origin. |
| Vector → Point | Same components as point | `[V]` quiz `cast-vector-point` | Interpretation changes, coordinates kept. |
| Text "Wall" → Number | Fails, blank | `[V]` quiz `cast-nonnumeric` | Red/blank. |
| Point → Mesh | Fails | `[V]` quiz `cast-point-mesh` | Needs an explicit mesh construction. |
| Text "Door" → Number | Fails | `[V]` quiz `cast-door-number` | A label does not encode a number. |
| Text "Room" → Integer | Fails | `[V]` quiz `cast-room-integer` | A separate lookup (e.g. Value List or dictionary) is not this cast. |
| Point → Plane | Plane at point, world XY axes | `[K]` | Common for "point as plane origin". `[CHECK]` in app. |
| Number → Integer | Decimal is rounded/truncated | `[K]` | Known to be lossy; `[CHECK]` rounding rule in app before stating it. |
| Integer → Boolean | 0 false, nonzero true | `[K]` | `[CHECK]`. |
| Text "1,2,3" → Point | Point (1,2,3) | `[K]` | Plain comma-separated numbers only. `[CHECK]`. |
| Line → Vector | Direction vector | `[K]` | `[CHECK]`. |
| Circle → Curve | Curve | `[K]` | Circle is a curve subtype: passes through. |
| Box → Brep | Brep | `[K]` | `[CHECK]`. |
| Curve → Line | Only if the curve is straight | `[K]` | Failure otherwise; use Explode/Line from ends. |
| Curve → Surface | Fails (not a surface) | `[K]` | Closed planar curve → surface needs Boundary Surfaces, not a cast. The brief's "closed planar" claim is a construction, not a cast. |
| Number → Domain | Domain only if the param expects one | `[CHECK]` | Do not teach this without checking. |

Rule for the clips: a cast is a type mapping on the wire. If the geometry does not exist, the cast cannot create it; another component must construct it (quiz: "A wire alone does not construct missing geometry").

### 2.3 Explicit conversions (use instead of relying on casts)
- Use a **Construct** component (e.g. Point XYZ, Line, Circle) to create geometry explicitly.
- Use **Parse** or **Text** components to convert text to numbers when the cast is ambiguous.
- Use **Deconstruct** (Point, Plane) to extract components.
- `[CHECK]` exact component names in Rhino 8 Params/Maths/Vector/Curve tabs before filming.

---

## 3. Data trees, branches and paths

### 3.1 Path syntax `[V]`
Source: McNeel Advanced Data Structures (https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/advanced-data-structures/).
- Branch path: integers in braces separated by semicolons, e.g. `{0;3;0}`.
- Item index: square brackets after the path, e.g. `{0;3;0}[1]`.
- Quiz examples use `{0}: [10, 20, 30]` (one branch, three items) and `{0;0}: [10]` (singleton branch).
- Teaching rule: a branch is one list, a path identifies it, an index addresses items within it.

### 3.2 Access modes (quiz vocabulary plus McNeel behaviour)
- **Item access**: a component processes one item at a time, branch by branch. Matching across branches is per component.
- **List access**: a component receives a whole list (one branch) and processes it as one unit.
- **Tree access**: a component receives the whole tree and returns a tree.
- `[K]` Component inputs are shown in the tooltip as "Item", "List" or "Tree" access (parameter setting). Teaching tip: set access on the input parameter, not on the output.

### 3.3 Tree operations (quiz vocabulary, with McNeel behaviour)

| Operation | What it does | Quiz example (verified against quiz `tree-operations`) |
|---|---|---|
| Flatten | Collects all branches into one list at `{0}`; grouping is lost. `[V]` | Room tree 1: `{0}: [10, 20, 30]` after Flatten. |
| Graft | Puts each item into its own branch; new last index = its former list index. `[V]` | Room tree 2: `{0;0}: [10]; {0;1}: [20]; {0;2}: [30]`. |
| Simplify | Removes a common leading path prefix; keeps branches separate; does not flatten. `[V]` | Room tree 3: `{0}: [10, 20]; {1}: [30]` becomes `{0}: [10, 20]; {1}: [30]` in the quiz wording. |
| Explode Tree | Returns each branch as a separate list. `[V]` | Use for handing branches to a list component. |
| List Item | Selects one index in every branch when applied to a tree. `[V]` | Index 0 of each branch. |
| Path Mapper | Rewrites paths by a mapping expression; source path is fixed, only target is set. Constants include `item_count`, `path_count`, `path_index`. Built-in mappings: Null, Flatten, Graft, Reverse, Renumbering. May cause data loss. `[V]` | Used to swap last two path integers (group corresponding branches), or swap branch index with item index (flip). |
| Entwine `[K]` | Combines several trees into one by adding a path prefix per input. | `[CHECK]` the exact behaviour before filming. |
| Tree Statistics `[K]` | Reports branch count, item count, paths. | `[CHECK]`. |
| Flip Matrix `[V]` | Turns an n-branch by m-item tree into m branches by n items; variable-length branches give nulls; fails on variable-depth trees. | (McNeel Advanced Data Structures.) |
| Parameter Viewer `[K]` | Shows the tree structure of an input, with paths as labels, for debugging. | Use with Panel to show paths. |
| Clean Tree / Trim Tree `[V]` | Removes null items and empty branches. | |

### 3.4 How tree matching works `[V]`
McNeel rules for tree-to-tree and list-to-tree matching (from Advanced Data Structures):
- **Item to tree**: the single item is repeated to match the tree's structure.
- **Short list to tree**: the list repeats in each branch; its last item repeats to fill gaps.
- **Long list to tree**: branches expand by repeating each branch's last item.
- **Tree to tree**: if one tree has fewer branches, its last branch repeats; corresponding branches then match lengths by repeating their last items.
- An extra depth level in one input can change results even when the trees look alike.
- Teaching implication: a component with item access matches branches first, then items within each branch. Mismatched depth is the most common hidden error.

### 3.5 Common mistakes (tree)
1. Confusing Flatten with Simplify (Flatten merges branches; Simplify keeps them).
2. Expecting Graft to keep the original grouping (it does not; each item gets a branch).
3. Assuming a Panel shows the list order only; it also shows the path, which is what matters for matching.
4. Treating `{0}` and `{0;0}` as the same structure.
5. Forgetting that item access works per branch, so a short branch receives repeated last items under matching rules.
6. Using Path Mapper without checking `path_count` and `item_count`; data loss is silent.
7. Believing a red component means every downstream result is wrong (only the first red node matters).

---

## 4. List matching (quiz `list-matching`, verified against McNeel)

Quiz categories and definitions:
- **Longest List**: repeats the last value of the shorter list until it matches the longer list. Quiz example: `(1,10) (2,20) (3,20)` (3 pairs; the shorter list's last value 20 is repeated).
- **Shortest List**: pairs matching indices only up to the shorter list's length; the longer list is trimmed at its end. Quiz example: `(1,10) (2,20)`.
- **Cross Reference**: every A item paired with every B item (Cartesian product). Quiz example: 3 × 2 = 6 pairs.

McNeel confirmation `[V]` (search results from wiki.mcneel.com and GH API docs):
- Shortest List: one-to-one pairing, stops when one input runs out.
- Longest List: runs until the longest input is exhausted; default.
- Cross Reference: all combinations; the GH API docs warn it can create very large data sets.
- Matching applies within a branch, not across branches; branch-level matching is handled by tree matching (§3.4).
- `[CHECK]` In Rhino 8 the mode is set via the Sets tab (List Length / Cross Reference components) or the List panel dropdown; confirm before filming.

Quiz note: the English term "Longest List" is kept as-is in DE in the quiz (`Longest List`, `Shortest List`, `Cross Reference`). Keep the same in both clip languages.

Common mistakes (list):
1. Thinking Longest List cycles through the shorter list (it repeats only the last value).
2. Thinking Shortest List keeps all items (it drops trailing items of the longer list).
3. Thinking Cross Reference pairs by index (it creates every combination).
4. Mixing list matching with branch matching.
5. Counting pairs incorrectly for Cross Reference (product, not sum).

---

## 5. Implicit iteration
- A component with item access runs once per matched pair (per item, per branch) without an explicit loop. This is what the quiz calls matching.
- `[K]` Example: Addition with two lists of equal length produces one result per index.
- Teaching warning: "implicit" means the matching rule decides the pairing; students must predict it, not assume index alignment.

---

## 6. Suggested term list (EN / DE)

| EN | DE (recommended) | DE alternative in quiz/usage |
|---|---|---|
| Definition / script | Definition | Skript |
| Canvas | Canvas | Arbeitsfläche |
| Component | Komponente | |
| Nickname | Kurzname | |
| Input | Eingang | |
| Output | Ausgang | |
| Wire | Verbindung | Draht |
| Parameter | Parameter | |
| Floating parameter / capsule | Freier Parameter (Kapsel) | Number-Kapsel (quiz) |
| Number Slider | Zahlenschieberegler (Slider) | |
| Panel | Panel | Textanzeige |
| Boolean Toggle | Boolescher Schalter | |
| Value List | Werteliste | |
| Plugin | Plugin | Erweiterungspaket |
| Package Manager | Paketverwaltung | |
| Data | Daten | |
| Item / element | Element | Item |
| List | Liste | |
| Branch | Ast | Zweig (quiz) |
| Tree | Datenbaum | |
| Path | Pfad | |
| Index | Index | |
| Casting | Typumwandlung | Cast |
| Item access | Elementzugriff | |
| List access | Listenzugriff | |
| Tree access | Baumzugriff | |
| Data matching | Datenabgleich | Datenverknüpfung |
| Longest List | Longest List | (keep English) |
| Shortest List | Shortest List | (keep English) |
| Cross Reference | Cross Reference | (keep English) |
| Flatten | Flatten | (keep English) |
| Graft | Graft | (keep English) |
| Simplify | Simplify | (keep English) |
| Path Mapper | Path Mapper | (keep English) |

Recommendation: keep UI names in English in both languages (Flatten, Graft, Simplify, Longest List, Shortest List, Cross Reference, Path Mapper), because students see them in the app. Translate concepts (branch, path, casting, panel, slider) in the caption.

---

## 7. Didactic misconceptions to address in the video (10)

1. A wire does not create geometry; a cast only maps data types (quiz cast prompt).
2. Text "12" becomes a number; text "Wall" does not (cast success vs failure).
3. Point → Vector changes interpretation (direction and magnitude), not coordinates.
4. Flatten destroys grouping; Simplify keeps branches.
5. Graft gives each item its own branch, and the new path index is the old list index.
6. Longest List repeats only the last value of the shorter list; it does not cycle.
7. Shortest List drops trailing items; it does not stop at the shorter list's first mismatch.
8. Cross Reference is a Cartesian product, not index pairing.
9. A panel shows paths; path order is the basis of tree matching.
10. Red component: trace upstream to the first red node; grey downstream is not a sign of correctness.

Additional items in the quiz for context: "An orange component is a warning, not a fail" `[K]`; "a plugin is a package, a component is one operation" (quiz canvas-terminology prompt).

---

## 8. Sources

- McNeel Developer Guides, Grasshopper Data Types (casting rules, CastTo/CastFrom): https://developer.rhino3d.com/guides/grasshopper/grasshopper-data-types/
- McNeel Developer Guides, Advanced Data Structures (tree paths, tree matching, Graft, Flatten, Simplify, Flip, Path Mapper): https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/advanced-data-structures/
- McNeel Developer Guides, Introduction to Data Structures (list matching section): https://developer.rhino3d.com/guides/grasshopper/gh-algorithms-and-data-structures/data-structures/
- McNeel Developer Guides, List Components: https://developer.rhino3d.com/guides/grasshopper/list-components/
- McNeel Developer Guides index (overview of data-tree and casting pages): https://developer.rhino3d.com/guides/grasshopper/
- McNeel wiki, Data stream matching algorithms: https://wiki.mcneel.com/labs/explicithistory/datastreammatchingalgorithms
- McNeel GH_DataComparison API reference (Shortest, Longest, Cross Reference): https://developer.rhino3d.com/api/grasshopper/html/T_Grasshopper_Kernel_GH_DataComparison.htm
- Grasshopper3D forum, "data matching explained": https://www.grasshopper3d.com/profiles/blogs/data-matching-explained
- Food4Rhino (plugin distribution): https://www.food4rhino.com (not fetched in this session; `[K]`)
- Quiz sources (repo): `🎓️teaching/🏛️architecture/🦗️grasshopper/❓️quiz/🔣️.json`; parametric context `🎓️teaching/🏛️architecture/📐️parametric/❓️quiz/🔣️.json`.

Not verified in this session (flagged `[CHECK]`): slider rounding labels in Rhino 8; exact cast rules for Number→Integer, Integer→Boolean, Text→Point, Point→Plane, Line→Vector, Box→Brep; Entwine and Tree Statistics behaviour; Package Manager menu location in Rhino 8. Verify these in the app before writing captions, because the brief asks for precise Rhino 8 / GH1 behaviour.
