# BIM Mixin Private Method Audit

Read-only AST scan; no drawing imports, scene rendering or source mutation.

## Duplicate Private Methods

### _pair
- Section._pair(self, look, item, fill: str, stroke: str) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\✂️section\🐍️.py:46`
- Plan._pair(self, look, item, fill: str, stroke: str) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\📋️plan\🐍️.py:35`
- Signatures: identical
- Bodies: identical

## Declared Drawing Composition

- ClipScene(_drafting.Drafting,_Savoye,_Axon,_panels.Panels,_Analysis,_engine.Scene) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🐍️.py`
- SavoyeOrtho(_Plan,_Section,_Detail) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🐍️.py`
- Savoye(_Axon,SavoyeOrtho) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🐍️.py`
- Drafting() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\📐️drafting\🐍️.py`
- Analysis() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🧮️analysis\🐍️.py`
- Panels() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🧾️panels\🐍️.py`
- Section(_s.SheetMixin) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\✂️section\🐍️.py`
- Plan(_s.SheetMixin) at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\📋️plan\🐍️.py`
- SheetMixin() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\📏️sheet\🐍️.py`
- Detail() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🔩️detail\🐍️.py`
- Axon() at `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🧊️axon\🐍️.py`
## Concurrent Ground-Helper Correction

The first text read observed the reported incompatible definitions: Section._ground(self,d,to,h0,h1) and Axon._ground(self,drawing,g,view). Between that read and the AST snapshot, concurrent authored edits removed the duplicate. Current definitions/calls are:

- `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\✂️section\🐍️.py:77 def _section_ground(self, d, to, h0, h1):`
- `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\✂️section\🐍️.py:138 self._section_ground(d, to, ctx["h0"], ctx["h1"])`
- `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🧊️axon\🐍️.py:728 self._ground(drawing, g, projection)`
- `🎓️teaching\🏛️architecture\🏗️bim\🎬️clip\🎨️visuals\🏛️savoye\🧊️axon\🐍️.py:805 def _ground(self, drawing, g, view):`

Under the declared Savoye(Axon,SavoyeOrtho) and SavoyeOrtho(Plan,Section,Detail) composition, the former self._ground call from Section would resolve Axon first. Current owner-qualified names avoid that collision. Plan precedes Section for the remaining _pair duplicate, but both implementations have identical signature and AST body, so no behavior mismatch was found. This was a filesystem AST/MRO-order audit, not a runtime constructor test.
