# Round 2 final verification (run 2)

Probe: `border-gap-probe.js`, `probe({mark:true})`, imported per view with `?t=`, re-run until `pending:false`. Acceptance: `count` 0.
Browser: own tab `tab-25` (other tabs untouched). Servers started natively on 5392 (staffage), 5393 (demonstrator), 5394 (presentation), 5395 (hub admin), 5396 (play harness), all from this run and killed at the end. Course 5391 and play 5397 belong to others and were not touched.

Method notes:
- Hub admin is served under base `/admin/`, so the probe must be imported as `/admin/@fs/...`. `/@fs/...` returns 404 there.
- The play landing starts with the introduction overlay. It was dismissed with Skip before probing. A pane resize reloads the page and brings the overlay back.
- Screenshots are returned at low resolution (about 442x276 at 1440, 129x279 at 375). Visual coordinates below are estimates in viewport px.

## Results

| App | View | Viewport | count | pending | Notes |
|---|---|---|---|---|---|
| staffage | overview `#/` | 1440x900 | 0 | false | |
| staffage | overview `#/` | 375x812 | 0 | false | |
| staffage | all assets `#/all` | 1440x900 | 0 | false | |
| staffage | all assets `#/all` | 375x812 | 0 | false | |
| staffage | category `#/humans` | 1440x900 | 0 | false | |
| staffage | category `#/humans` | 375x812 | 0 | false | |
| staffage | detail `#/humans/cyclist` | 1440x900 | 0 | false | |
| staffage | detail `#/humans/cyclist` | 375x812 | 0 | false | |
| staffage | collections `#/collections` | 1440x900 | 0 | false | |
| staffage | collections `#/collections` | 375x812 | 0 | false | |
| staffage | compare `#/compare` | 1440x900 | 0 | false | |
| staffage | compare `#/compare` | 375x812 | 0 | false | |
| staffage | playground `#/playground` | 1440x900 | 7 | false | violations below |
| staffage | playground `#/playground` | 375x812 | 0 | false | |
| demonstrator | overview, introduction shown | 1440x900 | 4 | false | intro box vs cards, below |
| demonstrator | overview, introduction shown | 375x812 | 4 | false | intro box vs cards, below |
| demonstrator | overview, introduction skipped | 1440x900 | 0 | false | |
| demonstrator | overview, introduction skipped | 375x812 | 0 | false | |
| presentation | slides 1-3 at rest | 1440x900 | 0 | false | |
| presentation | slide 3, one disposition selected | 1440x900 | 0 | false | `sel` 1 |
| presentation | slide 2 at rest (scroll view) | 375x812 | 0 | false | |
| presentation | selected disposition | 375x812 | n/a | n/a | not reached: the click did not select (`sel` 0) |
| hub admin | overview `/admin/` | 1440x900 | 0 | false | |
| hub admin | spaces | 1440x900 | 0 | false | |
| hub admin | overview `/admin/` | 375x812 | 0 | false | |
| hub admin | spaces | 375x812 | 0 | false | |
| semio-tech play | landing, intro skipped | 1440x900 | 1 | false | pair below, likely hover |
| semio-tech play | landing, swipe (intro skipped) | 375x812 | 0 | false | |

## Violations (verbatim probe output)

### staffage playground, 1440x900 (count 7)
```
5 div.flex.items-center.border-t «Walking Commuter» css:top || div.absolute.top-1.h-6 «» css:top
5 div.absolute.top-1.h-6 «» css:bottom || div.flex.items-center.border-t «Seated Reader» css:top
5 div.flex.items-center.border-t «Seated Reader» css:top || div.absolute.top-1.h-6 «» css:top
5 div.absolute.top-1.h-6 «» css:bottom || div.flex.items-center.border-t «Walking Commuter» css:top
5 div.absolute.top-1.h-6 «» css:bottom || div.flex.items-center.border-t «Sleeping Person» css:top
5 div.flex.items-center.border-t «Sleeping Person» css:top || div.absolute.top-1.h-6 «» css:top
1 div.absolute.top-1.h-6 «» css:right || div[slot=shared-border-row].relative.flex.flex-col «0 m5 m10 m15 m20 m Walking Com» css:right
```

### demonstrator overview, introduction shown, 1440x900 (count 4)
```
33.38 div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:h || div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «AggregatorAssemble existing st» svg:h
19.05 div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v || div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «EditAdapt partsEdit plans the » svg:v
18.83 div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «EditAdapt partsEdit plans the » svg:v || div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v
28.66 div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v || div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «AggregatorAssemble existing st» svg:v
```

### demonstrator overview, introduction shown, 375x812 (count 4)
```
16 button.ui-glass.absolute.z-[32] «Energy» svg:v || div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v
11 div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v || div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «GeneratorParametric flowsIn th» svg:v
11 div[slot=demonstrator-pane-card-stack].relative.flex.min-h-0 «GeneratorParametric flowsIn th» svg:v || div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v
16 div[slot=introduction-info-box].flex.min-h-0.min-w-0 «Welcome to Designing with Exis» svg:v || button.ui-glass.absolute.z-[32] «Coordinator» svg:v
```

### semio-tech play landing, intro skipped, 1440x900 (count 1)
```
26.39 div[slot=play-pane-card-stack].relative.flex.min-h-0 «Eurocode 2Concrete structuresO» svg:h || div[slot=play-pane-card-stack].relative.flex.min-h-0 «MathWrite equationsOpen» svg:h
```
The pointer rested on the "Eurocode 2" card after the Skip click. The 26 px offset matches the card's hover lift (`group-hover:-translate-y-0.5` interlock), so this is most likely a hover-state artifact. Not re-verified with the pointer away.

## Visual defects (screenshots, estimated viewport coordinates)

1. **Staffage playground, 1440x900.** The probe's magenta outlines sit on the figure picker rows (Walking Commuter, Seated Reader, Sleeping Person). They are the 5 px gaps between each row's top border and the absolute `top-1 h-6` markers, at about x 30-1110, y 770-840. The last pair is a 1 px edge at the right of the shared-border row. Screenshot resolution was too low to confirm the rendering beyond the probe outlines.
2. **Hub admin spaces, 375x812.** The tab bar overflows: "Observab…" is clipped at the right viewport edge (y about 0-56). The spaces table is wider than the viewport: the Actions column (Rename, Toggle visibility, Archive, Delete) is cut off at x about 365+, y about 93-212. The Name column wraps "Space 1" onto two lines. The tab overflow was already known (previous run), and the table overflow was not previously recorded.
3. **Demonstrator intro, 1440x900 and 375x812.** The introduction info box is not flush with the card stack it overlaps. At 375 the "Energy" and "Coordinator" glass chips meet the box edge at a 16 px step. At 1440 the box bottom sits 33 px from the "Aggregator" card line. Probe count is 4 at both sizes, and 0 once the intro is skipped.
4. **Play landing, 375x812 (swipe).** The "CAD" title chip and the header band to its right sit about 1 px below the navbar's bottom line, which reads as a near-double horizontal line at about y 56-58, x 85-348. The card's left edge is at about x 28, and the bottom "Open" chip's left edge at about x 26-27, a 1-2 px misalignment. The probe reports 0 for these.
5. **Presentation, 1440x900, slide 3 with selection.** Two magenta-outlined squares appear at the bottom right, about x 1332-1440, y 820-860. The probe reports count 0 and does not mark them, so they are not probe outlines. Origin not identified.

## Not verified / limits

- Presentation at 375 did not reach a selected disposition. The Reveal scroll view showed a scaled-down deck grid and the click did not select (`sel` 0).
- Play landing at 1440 was probed with the pointer resting on a card; the one pair above is most likely hover. Play with the intro shown (modal over the cards) was not counted, per the brief's exclusion of modal overlays.
- Staffage overview at 1440 was probed as `#/` at the start of the run (count 0); the 375 overview is also 0.
- Overview `#/all` and playground were the only staffage views where a screenshot was taken per viewport (playground only); other staffage views have probe results only.
- Demonstrator opened panes, healthy pane interiors and German locale were not run.

## Servers

Started by this run and killed at the end: staffage 5392, demonstrator 5393, presentation 5394, hub admin 5395, play harness 5396. Logs: `🗑️generated/v2-*.log`.
