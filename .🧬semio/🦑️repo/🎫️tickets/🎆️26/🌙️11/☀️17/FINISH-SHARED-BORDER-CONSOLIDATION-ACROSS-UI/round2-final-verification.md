# Round 2 final verification (live border-gap probe)

Status: PARTIAL. Only the course app was probed. The built-in browser pane was hidden for the rest of the run (`tabs_select` did not display it, screenshots timed out, probes never committed frames), so staffage, demonstrator, presentation, hub admin and semio-tech play were NOT verified in this run. Servers for those were started and are listed below for a re-run.

Probe: `border-gap-probe.js` via `await import('/@fs' + encodeURI(ABS) + '?t=' + Date.now())`, `probe({mark:true})`. Acceptance: `count` 0 and `pending` false.

Method notes:
- `pending` stayed true on `#energy` until the tab was composited. `tabs_select` plus a screenshot (forces a frame) made `pending` false. Results below are only taken with `pending: false`.
- Course data sometimes showed "The course server cannot be reached right now" for a few seconds after navigation. Probes were taken after the content loaded.
- Probes were run with `mark:true`, so offenders are outlined in magenta in the pane.

## Results

| App | View | Viewport | count | pending | Notes |
|---|---|---|---|---|---|
| course | home (`/?preview=1`) | 1440x900 | 39 | false | violations listed below |
| course | home (`/?preview=1`) | 375x812 | 4 | false | violations listed below |
| course | `#energy` | 1440x900 | 12 | false | violations listed below |
| course | `#energy` | 375x812 | 11 | false | violations listed below |
| course | `#physics` | 1440x900 | 0 | false | |
| course | `#physics` | 375x812 | 0 | false | |
| course | run view (physics, Medium resumed) | 1440x900 | 0 | false | |
| course | run view (physics, Medium resumed) | 375x812 | 0 | false | |
| course | "Discard the open run?" dialog (after Start Easy) | 375x812 | 5 | false | transient modal over the page, listed below |
| course | settings (`#prefs`) | 1440x900 | 0 | false | |
| course | settings (`#prefs`) | 375x812 | 0 | false | |
| course | results page | - | - | - | not reached |
| staffage | all views | - | - | - | NOT PROBED (pane hidden) |
| mit-bestand demonstrator | overview, opened panes | - | - | - | NOT PROBED (pane hidden) |
| presentation | slides at rest / selected | - | - | - | NOT PROBED (pane hidden) |
| hub admin | gate, overview, spaces | - | - | - | NOT PROBED (pane hidden) |
| semio-tech play | language choice, landing | - | - | - | NOT PROBED (pane hidden) |

Verdict so far: course physics, run view, and settings are clean at both viewports. Course home (both viewports) and energy (both viewports) are NOT clean.

## Violations (verbatim probe output)

### course home, 1440x900 (count 39)

```
7.05 th.border-b.border-normal.px-single «BIM Concepts» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Anonymous #81cb82aePoints: 01 » svg:h
7.05 td.border-b.border-normal.px-single «In progress» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Anonymous #81cb82aePoints: 01 » svg:h
8.05 td.border-b.border-normal.px-single «Hard» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «QuizzesExplore quizzes by topi» svg:h
7.05 td.border-b.border-normal.px-single «–No score» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «QuizzesExplore quizzes by topi» svg:h
7.05 td.border-b.border-normal.px-single «–No points» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «QuizzesExplore quizzes by topi» svg:h
7.05 td.border-b.border-normal.px-single «Nov 17, 2026, 5:50 PM» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «QuizzesExplore quizzes by topi» svg:h
8.05 td.border-b.border-normal.px-single «Nov 17, 2026, 5:50 PM» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «AssignmentsExercises with thei» svg:h
7.05 td.border-b.border-normal.px-single «–Not submitted» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «AssignmentsExercises with thei» svg:h
7.05 td.border-b.border-normal.px-single «Resume (Hard)» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «AssignmentsExercises with thei» svg:h
1.67 th.border-b.border-normal.px-single «Cooling» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Learning ClipsShort learning v» svg:h
1.67 td.border-b.border-normal.px-single «In progress» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Learning ClipsShort learning v» svg:h
1.67 td.border-b.border-normal.px-single «–No score» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h
1.67 td.border-b.border-normal.px-single «–No points» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h
1.67 td.border-b.border-normal.px-single «Nov 17, 2026, 5:34 PM» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h
1.67 td.border-b.border-normal.px-single «–Not submitted» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Badges🧲 Physics Expert: Not y» svg:h
1.67 td.border-b.border-normal.px-single «Resume (Easy)» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Badges🧲 Physics Expert: Not y» svg:h
6.2 div.flex.max-w-full.w-full «Today This week This month All» css:bottom || td.border-b.border-normal.px-single «Hard» css:bottom
6.2 div.flex.max-w-full.w-full «Today This week This month All» css:bottom || td.border-b.border-normal.px-single «–No score» css:bottom
6.2 div.flex.max-w-full.w-full «Today This week This month All» css:bottom || td.border-b.border-normal.px-single «–No points» css:bottom
6.2 div.flex.max-w-full.w-full «Today This week This month All» css:bottom || td.border-b.border-normal.px-single «Nov 17, 2026, 5:34 PM» css:bottom
6.2 div «🧲 Physics Expert: Not yet ear» css:bottom || td.border-b.border-normal.px-single «Nov 17, 2026, 5:34 PM» css:bottom
6.2 div «🧲 Physics Expert: Not yet ear» css:bottom || td.border-b.border-normal.px-single «–Not submitted» css:bottom
6.2 div «🧲 Physics Expert: Not yet ear» css:bottom || td.border-b.border-normal.px-single «Resume (Hard)» css:bottom
0.3 td.border-b.border-normal.px-single «Easy» css:bottom || tr.border-t.border-normal.course-me «1Anonymous #81cb82ae (you)00» css:top
0.3 td.border-b.border-normal.px-single «–No score» css:bottom || tr.border-t.border-normal.course-me «1Anonymous #81cb82ae (you)00» css:top
0.3 td.border-b.border-normal.px-single «–No points» css:bottom || tr.border-t.border-normal.course-me «1Anonymous #81cb82ae (you)00» css:top
0.3 td.border-b.border-normal.px-single «Nov 17, 2026, 5:33 PM» css:bottom || tr.border-t.border-normal.course-me «1Anonymous #81cb82ae (you)00» css:top
3.14 div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h || td.border-b.border-normal.px-single «Easy» css:bottom
3.14 div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h || td.border-b.border-normal.px-single «–No score» css:bottom
3.14 div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h || td.border-b.border-normal.px-single «–No points» css:bottom
3.14 div[slot=course-card-stack].relative.flex.min-h-0 «Leaderboard Today This week Th» svg:h || td.border-b.border-normal.px-single «Nov 17, 2026, 5:10 PM» css:bottom
3.14 div[slot=course-card-stack].relative.flex.min-h-0 «Badges🧲 Physics Expert: Not y» svg:h || td.border-b.border-normal.px-single «» css:bottom
2.14 div[slot=course-card-stack].relative.flex.min-h-0 «Learning ClipsShort learning v» svg:h || td.border-b.border-normal.px-single «Voided» css:bottom
2.14 div[slot=course-card-stack].relative.flex.min-h-0 «Learning ClipsShort learning v» svg:h || td.border-b.border-normal.px-single «Easy» css:bottom
1.14 div[slot=course-card-stack].relative.flex.min-h-0 «SettingsLanguage, colour theme» svg:h || th.border-b.border-normal.px-single «Energy Demand» css:bottom
1.14 div[slot=course-card-stack].relative.flex.min-h-0 «SettingsLanguage, colour theme» svg:h || td.border-b.border-normal.px-single «Voided» css:bottom
6.25 th.border-b.border-normal.px-single «Objectives and Pareto» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «SettingsLanguage, colour theme» svg:h
6.25 td.border-b.border-normal.px-single «In progress» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «SettingsLanguage, colour theme» svg:h
20.31 div[slot=course-card-stack].relative.flex.min-h-0 «SettingsLanguage, colour theme» svg:h || th.border-b.border-normal.px-single «BIM Concepts» css:bottom
```

### course home, 375x812 (count 4)

```
3.34 div[slot=course-card-stack].relative.flex.min-h-0 «Anonymous #81cb82aePoints: 01 » svg:h || tr «CoolingIn progressEasy–No scor» css:bottom
3.34 button.ui-glass.absolute.z-[32] «Assignments» svg:h || tr «CoolingIn progressEasy–No scor» css:bottom
3.34 button.ui-glass.absolute.z-[32] «Quizzes» svg:h || tr «CoolingIn progressEasy–No scor» css:bottom
2.34 button.ui-glass.absolute.z-[32] «Learning Clips» svg:h || tr «CoolingIn progressEasy–No scor» css:bottom
```

### course `#energy`, 1440x900 (count 12)

```
22.02 li «2☀︎From Tea Light to SunSortin» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h
22.02 li «2☀︎From Tea Light to SunSortin» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «🌬︎VentilationVentilation for » svg:h
22.02 li «2☀︎From Tea Light to SunSortin» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «🔥HeatingU-values of building » svg:h
4.05 div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «Tasks in this quiz1⚡Power or E» svg:h
3.05 div[slot=course-card-stack].relative.flex.min-h-0 «🌬︎VentilationVentilation for » svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «Tasks in this quiz1⚡Power or E» svg:h
3.05 div[slot=course-card-stack].relative.flex.min-h-0 «🔥HeatingU-values of building » svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «Tasks in this quiz1⚡Power or E» svg:h
2.44 div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h
2.44 div[slot=course-card-stack].relative.flex.min-h-0 «🌬︎VentilationVentilation for » svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h
1.44 div[slot=course-card-stack].relative.flex.min-h-0 «📊Energy DemandRecognise energ» svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h
31.36 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:v || button.box-border.justify-center.overflow-visible «Resume (Medium)» css:right
31.86 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:v || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:v
17.7 div[slot=course-card-stack].relative.flex.min-h-0 «🌬︎VentilationVentilation for » svg:v || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:v
```

### course `#energy`, 375x812 (count 11)

```
7.61 div[slot=course-card-stack].relative.flex.min-h-0 «Tasks in this quiz1⚡Power or E» svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h
38.41 div[slot=course-card-stack].relative.flex.min-h-0 «Tasks in this quiz1⚡Power or E» svg:h || button.ui-glass.absolute.z-[32] «Heating» svg:h
20.67 div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h
11.13 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h
11.13 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h || button.ui-glass.absolute.z-[32] «Ventilation» svg:h
12.13 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h || button.ui-glass.absolute.z-[32] «Cooling» svg:h
20.97 div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h || button.box-border.overflow-visible.outline-none «Show it now» css:bottom
20.97 button.ui-glass.absolute.z-[32] «Heating» svg:h || button.box-border.overflow-visible.outline-none «Show it now» css:bottom
20.97 button.ui-glass.absolute.z-[32] «Ventilation» svg:h || button.box-border.overflow-visible.outline-none «Show it now» css:bottom
19.97 button.ui-glass.absolute.z-[32] «Cooling» svg:h || button.box-border.overflow-visible.outline-none «Show it now» css:bottom
22.83 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:v || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:v
```

### course "Discard the open run?" dialog, 375x812 (count 5, transient modal)

```
11.47 fieldset.course-kinds.m-0.min-w-0 «ChallengeEasyThe values show a» css:bottom || div[slot=course-card-stack].relative.flex.min-h-0 «Discard the open run?Your run » svg:h
35.05 div[slot=course-card-stack].relative.flex.min-h-0 «Discard the open run?Your run » svg:h || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:h
2.66 div[slot=course-card-stack].relative.flex.min-h-0 «What everyone answeredShow it » svg:h || footer.course-footer.relative.flex «Show petsWhat is stored» svg:h
8.8 div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:v || div[slot=course-card-stack].relative.flex.min-h-0 «Discard the open run?Your run » svg:v
8.8 div[slot=course-card-stack].relative.flex.min-h-0 «Discard the open run?Your run » svg:v || div[slot=course-card-stack].relative.flex.min-h-0 «🧲Physical UnderstandingTell p» svg:v
```

## Visual observations (course only, from the pane screenshots at 0.5 scale)

- 375 physics (`#physics`, the quiz view): the card stack reads as one column of bordered panels with shared edges. No visible double lines on the quiz card, the task rows, or the "What everyone answered" band. Clean, matching the probe.
- 375 run view (Medium resumed): the "Your run is on Medium" box and the task list sit flush on one another. No gap visible. Clean, matching the probe.
- 375 energy (`#energy`): the side chips "Heating", "Ventilation", "Cooling" sit on the card-stack edges with a visible step, and the "Resume (Medium)" / "Show it now" bands are offset from the stack lines. Magenta outlines confirm the offenders. Not consolidated.
- 375 home: "Assignments" and "Quizzes" side chips and "Learning Clips" bottom chip are outlined against the stack edge, with a visible seam/offset under the stack. Not consolidated.
- 1440 home and energy: not captured as a readable screenshot (the emulated 1440 frame renders at reduced scale in the pane). Probe counts (39 and 12) are the evidence; the home table rows (`td.border-b`) sitting directly under the card stack are the visible seams.
- Settings at 375 and 1440: segmented controls and toggles read as flush rows. Clean.

## Not verified in this run

- Staffage (port 5392, probe recipe in `round2-staffage.md`): server running, not probed.
- Demonstrator overview (port 5393), presentation (5394), hub admin (5395), semio-tech play (own server on 5396, harness per `round2-presentation-hub-site.md`): servers running, not probed.
- Course results page: not reached.
- Browser pane was hidden for the rest of the run. The person must show the Browser pane in the Claude desktop app (Cmd+Shift+B or the globe icon) before the rest of the probes can run.

## Servers

Started by this run (killed at the end of the run): staffage 5392, demonstrator 5393, presentation 5394, hub admin 5395, play (own) 5396. Logs in `🗑️generated/v-*.log`. Not touched: course 5391 (already running, not started here) and play 5397 (already running, not started here).
