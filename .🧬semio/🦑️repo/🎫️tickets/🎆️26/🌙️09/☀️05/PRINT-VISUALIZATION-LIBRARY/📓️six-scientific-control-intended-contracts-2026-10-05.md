# Six Scientific Control Intended Contracts — 2026-10-05

This report records exact current documentation/defaults and existing painter coordinates. Where the original prose does not determine a full algorithm, the execution owner must declare the precise schema-first neutral contract before implementing rather than attribute an invented formula to the old source. Exact current source snapshots are retained under native-unused-controls-source-before.

| Family / Control | Native Declaration / Setter | Documented Behavior | Existing Default Fixture | Current Renderer |
|---|---|---|---|---|
| sci-bracket / rounds | geometry1115 /1127 | Knockout rounds; comment1112 default3 | entries A..H, results 2/1/A,2/2/D,2/3/E,2/4/H,3/1/A,3/2/H,4/1/A; slotHeight6, roundWidth22 |1135 emits entries round1 and every authored result round/slot; coordinates x=(round-1)*roundWidth and y=-((slot-.5)*2^(round-1)-.5)*slotHeight; never reads rounds |
| sci-spacetime / gridLines | physics357 /368 | Primed grid density; comment353 default4 | mode minkowski,beta.5,window -2..2 bothaxes,worldlines0/A,.3/B,-.7/C,events.8/1.2/E1,-1.1/.6/E2 |370+ computes frame/null cones, boosted axes and worldlines; no primed-grid painter |
| sci-staff / beams | signal446 /457 | Join consecutive eighth notes; comment441 defaulttrue | clef treble,spacing5.5,stride7,events0/1/n,2/1/n,4/.5/s,5/.5/n,7/2/n,4/1/f,2/1/n,0/2/n |461 draws staff+clef; note handler positions xcursor, y=pitch*spacing/2, draws head/stem then cursor+=stride*duration; no beam grouping |
| sci-constellation / span | engineering584 /596 | Eye diagram symbol span; comment580 default2 | mode constellation,order16,noise.06,symbols240,span2,samples32,window -1.3..1.3 | Eye638 overlays24 seeded polarity/jitter traces; sample t=-1.3+2.6*i/samples, cosine squared(pi*t/2.6); span never read |
| sci-mohr / pole | engineering845 /856 | Draw pole construction; comment839 defaultfalse | sigmaX80,sigmaY20,tauXY30,principaltrue,polefalse; auto window from circle |859+ centre=(sigmaX+sigmaY)/2,radius=sqrt(((sigmaX-sigmaY)/2)^2+tauXY^2); draws circle, optional principal points and A=(sigmaX,tauXY),B=(sigmaY,-tauXY) diameter; pole never read |
| sci-mesh / fillCells | geometry835 /848 | Shade every cell by its own index; comment833 defaultfalse | mode triangulation,columns8,rows6,size6,warp1.5*sin(SemioX/3),axesfalse,gridfalse |852 loops48cells; node x=column*size,y=row*size+warp; cell881 draws bottom/right segments plus diagonal for triangles; fillCells never read |

Stock kinds: knockout-bracket authors rounds4/slotHeight5; tournament-bracket rounds3. Default eight entries and round4 winner illustrate three elimination stages plus the first entrant column; neutral rounds policy should explicitly distinguish stage count from displayed column number. New round selection should be tested against authored results beyond the requested stage, not only a stock default where both values happen to show the same supplied records.

Grid density should produce actual transformed primed-grid lines for nonzero beta in minkowski mode. Neutral authored coordinate fixtures must state the primed coordinate convention and compare endpoints independently; default background grid flag must not stand in for the source-owned primed grid density control.

The two consecutive .5-duration staff events are an exact default beam counterexample. Test beams=false/true with independently declared stem-tip positions and one connecting beam. Consecutive grouping, interruptions and isolated eighth flags must be explicit neutral rules, not merely path-count change.

Eye span should change the temporal sample domain/resolution consistent with documented symbols. Current literal2.6 temporal window cannot establish symbol-span semantics. Preserve the seeded polarity/jitter kernel separately, and author the sample-time formula for at least two nondefault spans independently of the native implementation.

Mohr pole documentation does not specify axis/shear sign or which reference plane line is used. The neutral fixture must define that convention using the already declared A/B stress points and validate circle incidence plus construction-line endpoints. A toggle that draws an arbitrary point would satisfy nonempty census but not pole semantics.

Mesh shading must create actual closed cell fill geometry with the fourth warped corner, per-cell index palette assignment and existing grid/diagonal strokes. Test both boolean states, quad and triangulation modes, a nonzero authored warp and non-square lattice with independently calculated vertices. Counting strokes alone cannot certify fillCells.

There are no direct exact family tokens for these six owners in the current print test features inspected; the broader adapter candidate map distinguishes indirect/generic routes. Existing numerical stress, path and field kernel probes can preserve unchanged math, but they do not validate these previously unread authored toggles.
