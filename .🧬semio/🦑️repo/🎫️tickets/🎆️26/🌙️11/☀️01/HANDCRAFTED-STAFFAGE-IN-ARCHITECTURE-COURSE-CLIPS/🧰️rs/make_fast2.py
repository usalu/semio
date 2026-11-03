import sys
from pathlib import Path
src = Path(sys.argv[1]).read_text(encoding="utf-8")
m = 'await page.clock.runFor(frame.transition_elapsed??1000);'
assert m in src
new = m + '''console.error("[ANIM] f"+index+" "+JSON.stringify(await page.evaluate(()=>document.getAnimations().map(a=>[a.constructor.name,a.playState,a.effect?.getComputedTiming().endTime,a.effect?.target?.tagName+"."+(a.effect?.target?.className?.toString?.().slice(0,30)),a.transitionProperty||a.animationName]).slice(0,8))));await page.evaluate(async()=>{await Promise.all(document.getAnimations().filter(a=>Number.isFinite(a.effect?.getComputedTiming().endTime)).map(a=>a.finished.catch(()=>{})))});'''
Path(sys.argv[2]).write_text(src.replace(m, new, 1), encoding="utf-8")
