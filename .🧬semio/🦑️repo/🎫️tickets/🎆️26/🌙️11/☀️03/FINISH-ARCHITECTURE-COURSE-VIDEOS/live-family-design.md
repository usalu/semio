# Live Family Render Boundary Investigation

Primary local Manim scene.py1384renders self.moving_mobjects after update_to_time;1714updatesanimations,mobjects,meshes,selfwithoutrebuildingthat list. get_moving_mobjects925returns flattened current family. Replacing children during a play leaves detached old children in the earlier list, explaining ghost overdraw.

Ticket-only prototype: an owned Scene mixin calls superclass update_to_time then recomputes moving_mobjects from current animations before renderer.render. It must pass actual slider/panel/variable-length text and mixed vector/image fresh-Cairo oracles, plus static background and true morph tests. Avoid scene.add_updater because its presence changes static Wait behavior. No installed external tool edits and no existing frozen keyed/snapshot/Nakagin changes are authorized. If proven, a new module may be imported only by the four stopped clients, preserving other active dependencies. Simultaneous reveal/updating is a separate suspension/timing issue requiring explicit tests. No production implementation or passing result is claimed yet.

## Prototype Results And Approved Integration

Ticket-onlymoving-familyprototypepassed4/4actualslider/panel(includingglyphlength),vectorandmixedimageframeoracles. RealvectorMorph,staticbackground,naturallyupdatingWaitandunchangedstaticWaitalsopassed. Testsidentifyandexcludeonlythepre-animationstaticbackgroundcapture,whilecheckingallwrittenframes. SimultaneousFadeIn+trackerremainswrongbecausewidgetupdatersaresuspended;authorsmustrevealthenramp. Actual120value-list/output/index/type-note timeline0.5sreveal+1.9srampretains2.4stotalandmatchesfreshCairoallninechangedframesexactly.

Approvedanewownedpuremixininadomain-neutralinteroplocation,importedonlyby120/121/123/126/128afterstoppingthefourloadedclientsandchecking128hasnotstarted. Energyownerimplementsmodule/permanentfixturetestsand121;drawingownerimplementsotherfourclientsandtimelines. Canonicalnon-targetfingerprintsbefore/aftercreationmustmatch. Noinstalledpackage,existingfrozenkeyed/snapshot/Sprite/Nakaginor19model/cachefileedits. No production passing result is claimed until the actual new module tests run.

## Production Interop Module

Newownedmodule: 🎓️teaching/🧰️interop/🎬️scene/🐍️.py. Permanenttests/schema/casesareunderits🧪️tests directory. CorrectedRefreshMovingusescurrentdrawablefamilyexcludingtheoriginalstaticfamily,andhandlesanemptymovingfamilywithoutdoublecompositing. SixpermanentnativeCairotests pass, includingmoving-boundaryopaque/translucentgeometry,actualwidgets,mixedvector/image,morphandWaitbehavior. Sixcanonicalnontargetsegmentfingerprints matchedbefore/afterfilecreation.

Rootregistered🎓️teaching.🧰️interop.🎬️scene.🧪️tests.🐍️ exactlyonceincanonicalauthoring📜script.ts;Node--checkpassed. Sourceclienttimingoraclesmustnowusetheproductionmodulebeforefreeze. SimultaneousFadeIn+trackerrequiresauthoredrevealthenramp;newinteropdoesnotclaimtooverrideanimationsuspension.
