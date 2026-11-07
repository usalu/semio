# Clip Readiness Tests

The browser clip capture waits for measured silhouette geometry. The shared compositor can split or remove individual border paths, so readiness must accept whole, split, and empty path lists while rejecting a pending border, a missing border, and stale content geometry. Portable JSON cases exercise these conditions through the extracted readiness predicate. The existing layered browser oracle opens a chapter through the production capture pipeline and independently exercises the real DOM.

The root readiness cases also cover missing compositors, initial and subsequent scheduled work, overview scenes without a window, and multiple roots. A compositor regression checks pending markers across successive animation frames and disposal. The focused Node test passed all ten named cases and the missing-border assertion.

The rendered layered browser oracle now checks pending work before and after the first animation frame and after a pointer-triggered reschedule. The geometry oracle accepts compositor-owned segments instead of assuming a single complete path. The rendered browser suite and compositor Vitest test could not be run: workspace Playwright, Tailwind, React, and Vitest dependencies were unavailable, and repository dependency setup stalled under capacity pressure. The layered browser oracle also drives production capture through an opened chapter once those dependencies are provisioned.

The production geometry oracle also requests a zoomed chapter, checks that its camera transform queues border composition and that composition is ready before capture, and compares the rendered images with Pillow. The registered Python runtime was also unavailable, so this additional browser case could not be executed here.

The clip contract places its focused window at y=32 through y=868, with the navbar ending at 28.8 and footer beginning at 871.2: both gaps are 3.2 pixels. Overview cells also have padding. Navbar and footer expose only their outer bottom and top edges as shared frames, while their title-chip buttons are borderless and do not register as frames. A production seam test would therefore assume geometry those scenes do not provide.

A separate synthetic browser fixture now places two actual WindowChrome components adjacent inside a clipped, scaled container. It measures the common edge, uses Chromium stroke hit testing to require one active owner and then one hovered owner, and checks the corresponding screenshot colors with Pillow. Its JavaScript passes Node syntax checking; the rendered test remains unrun until browser dependencies and Python are available.

The isolated focused chapter must expose at least one border segment. The synthetic raster check also rejects the losing state color near the measured seam, in addition to finding the winning color. Its browser launch uses the same repository-managed Chromium provisioning fallback as production. Cancellation aborts provisioning, closes an active browser and returns exit code 130; adapter cleanup also runs if launch fails. The updated JavaScript passed Node syntax checking, and the ten focused readiness cases passed again. The registered Python and uv tools were unavailable, so the rendered test remains unrun.

A further source review found no additional concrete harness defect. The seam syntax check and all ten readiness cases passed again; the browser packages remained unavailable while the existing setup continued.

A later cancellation review found that an uncaught cancellation exception overwrites Node's exit code 130 with 1. The harness now suppresses errors only when its cancellation signal is aborted, preserves other failures, and removes its adapter even if browser close fails. The changed script passes Node syntax checking; browser execution remains blocked on dependencies.

The default clip output directory `.🧬semio/🎓️teaching/⚡️cache` was absent, and the tracked energy clip directory contained no PNG, MP4, or WebM outputs. No existing clip frame was available for direct verification while browser dependencies remained unavailable.

Registered Node ran the actual `resolveBorderSegments` implementation against all four existing language-neutral border fixture cases, both in fixture order and reversed input order. All eight comparisons passed. This checks the resolved edge vectors and order independence; it does not replace rendered clip or pixel verification.

```powershell
repo tools exec --light node -- -e "const fs=require('fs'),assert=require('assert/strict'),p='./🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/',m=require(p+'🟦️.ts'),cases=JSON.parse(fs.readFileSync(p+'cases.json','utf8'));for(const c of cases){assert.deepEqual(m.resolveBorderSegments(c.input),c.expected,c.name);assert.deepEqual(m.resolveBorderSegments([...c.input].reverse()),c.expected,c.name)}console.log('passed',cases.length,'neutral border cases in both orders')"
```

Output: `passed 4 neutral border cases in both orders`. This invocation creates no temporary files.

The following invocation ran the actual shared silhouette projection against its two neutral cases and the actual junction resolver against three neutral cases in both input orders. All comparisons passed. It creates no temporary files and does not verify rendered pixels.

```powershell
repo tools exec --light node -- -e "const fs=require('fs'),assert=require('assert/strict'),p='./🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/',m=require(p+'shared-border.ts'),v=JSON.parse(fs.readFileSync(p+'shared-border.cases.json','utf8')).cases,j=JSON.parse(fs.readFileSync(p+'junctions.cases.json','utf8')).cases;for(const c of v){const a=[c.from,c.to].map(x=>m.projectSharedBorderPoint(x,...c.logical,...c.paint));assert.deepEqual(a,c.expected,c.name)}for(const c of j){const s=c.rectangles.flatMap(([l,t,w,h,sx,sy],i)=>m.sharedOutlineSegments(String(i),[{x:0,y:0},{x:w,y:0},{x:w,y:h},{x:0,y:h}],[{x:1,y:1},{x:w-1,y:1},{x:w-1,y:h-1},{x:1,y:h-1}],'normal',{left:l,top:t,scaleX:sx,scaleY:sy})),p={x:c.point[0],y:c.point[1]},expected={x:c.expected[0],y:c.expected[1]};assert.deepEqual(m.sharedBorderJunction(p,s),expected,c.name);assert.deepEqual(m.sharedBorderJunction(p,s.reverse()),expected,c.name)}console.log('passed',v.length,'projection cases and',j.length,'junction cases in both orders')"
```

Output: `passed 2 projection cases and 3 junction cases in both orders`.

Registered Node also ran the actual clipping functions against the existing hidden-inset and clipped-active-owner regressions. Both passed; this checks eligibility before paint resolution rather than rendered pixels. No temporary files were created.

```powershell
repo tools exec --light node -- -e "const assert=require('assert/strict'),m=require('./🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.ts'),logical=[{x:0,y:0},{x:100,y:0},{x:100,y:100},{x:0,y:100}],paint=[{x:1,y:1},{x:99,y:1},{x:99,y:99},{x:1,y:99}],left=m.sharedOutlineSegments('left',logical,paint,'active',{left:0,top:0,scaleX:1,scaleY:1}).map(s=>({...s,clip:{left:0,top:0,right:99.5,bottom:100}})),right=m.sharedOutlineSegments('right',logical,paint,'normal',{left:100,top:0,scaleX:1,scaleY:1}),eligible=m.visibleSharedOutlineSegments([...left,...right]);assert(!eligible.some(s=>s.owner==='left'&&s.from.x===100&&s.to.x===100));assert(eligible.some(s=>s.owner==='right'&&s.from.x===100&&s.to.x===100));assert.deepEqual(m.clipSharedOutlineSegments(right,{left:0,top:0,right:100,bottom:100}),[]);console.log('passed clipped owner exclusion and viewport-boundary visibility')"
```

Output: `passed clipped owner exclusion and viewport-boundary visibility`.

An actual rendered shared seam still needs a pixel-level regression. The focused chapter window is isolated, and overview cards may have intentional gaps; navbar or footer chip cells are likely candidates. Once browser dependencies are installed, measure frame rectangles and select a pair whose edges actually coincide. Assert that exactly one compositor segment covers an interior point of that shared edge and compare the screenshot pixel with its resolved stroke color using Pillow, allowing for subpixel antialiasing. Repeat after a hover or active-state change to check ownership transfer. Do not assume overview cards touch without measuring them.
