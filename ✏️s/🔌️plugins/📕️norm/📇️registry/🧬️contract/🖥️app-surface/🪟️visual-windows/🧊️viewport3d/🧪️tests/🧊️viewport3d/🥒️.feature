@capability-viewport3d-lowering
@oracle-python-section-areas
@oracle-python-dynamics
@comparison-closed-form-and-shapely
Feature: Structural scene lowering to the 3D viewport
  A locale-free structural scene lowers deterministically to the framework world-3d lanes: instanced meshes keyed by section,
  utilisation colours with a legend, highlights, section planes, annotations in both languages and device-dependent detail.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Section prism volume equals the oracle area for <section>: Given the oracle golden "🧫️fixtures/🔣️prisms.json" computed by "🔮️oracles/🐍️.py" When the unit prism of the "<section>" outline is meshed Then its enclosed volume equals the oracle area within 1e-9 Examples (3 rows: ishape, rhs, chs)
  - A beam along +X lowers to one instance with the golden transform: Given a scene with one steel member "b1" from (0,0,0) to (4,0,0) in the world frame When the scene is lowered with the default configuration Then there is 1 instance with scale [1,1,4], rotation [0.5,0.5,0.5,0.5] and granularity "member" And its id decodes to the scoped identity ["member","b1"]
  - Equal sections and equal utilisation buckets share one mesh: Given three members of equal section with utilisations 0.64, 0.64 and 1.10 When the scene is lowered Then there are 3 instances and 2 meshes
  - A 10 by 10 bolt grid is instanced from two meshes: Given a hex-head bolt pattern of 10 by 10 on layer "fasteners" When the scene is lowered on a desktop device Then there are 200 instances and 2 meshes When the scene is lowered on a mobile device Then there are 100 instances
  - The device budget truncates and says so: Given a bolt pattern of 60 by 60 When the scene is lowered on a mobile device Then the truncated count is 1 And the description contains "hidden to stay within the device budget"
  - The worst member drives the scalar field and legend: Given a member with station utilisations 0.5 at the start and 1.12 at the end When the scene is lowered Then the scalar field range is 0 to 1.2 And its values span 0.5 to 1.12 over the member mesh vertices
  - Dimension text follows locale and unit: Given a dimension of 3.2 m Then the English text is "3.2 m" and the German text is "3,2 m" And with the millimetre unit the English text is "3200 mm"
  - Picking resolves to entity id and document field path: Given the scoped address of member "b1" with the part suffix "#3" When it is resolved Then the entity id is "b1" and the field path is "member[id=b1]"
  - Selection is announced for assistive technology: Given a member "b1" with utilisation 0.64 that passes Then the English announcement is "Entity b1 selected, utilisation 64 %, passes" And the German announcement is "Objekt b1 ausgewählt, Ausnutzung 64 %, besteht"
  - Layouts exist for desktop, mobile and tablet: When the visual layout is built for each device class Then the desktop layout is a row of three areas with viewport in the middle And the mobile layout is one stack of four tabs with the viewport active And the tablet layout is a row of viewport and a three-tab stack
  - Oscillation phase and ramp follow the oracle: Given the oracle golden "🧫️fixtures/🎬️dynamics.json" computed by "🔮️oracles/🎬️dynamics/🐍️.py" When the transport clock is 0, 0.5, 1 and 1.5 seconds while playing Then the harmonic phase is 0, 1, 0 and -1 within 1e-12 When the transport clock is 0, 1, 2 and 3 seconds while playing Then the ramp factor is 0, 0.5, 1 and 1
  - Reduced motion never starts an animation: Given the user prefers reduced motion When play is requested and the clock ticks Then the transport stays paused and the clock stays 0 And the mode shape is drawn at the manual phase And seeking and stepping the parameter still work
  - A fire duration track blends temperature fields: Given the track "fire" from 0 to 3600 s with keyframes at 0, 1800 and 3600 s When the parameter is 900 s Then the fields blend "temp@0" and "temp@30" with weight 0.5 And the legend range spans the minimum and maximum over all keyframe fields And the readout is "Fire duration: 15 min" and "Branddauer: 15 min"
  - The toolbar picks the active field: Given a member with the fields "settlement" and "stress" When the active field is "stress" Then the legend is titled "Stress" with unit "MPa" and the ramp is baked per vertex
  - Plate and shell eigenmodes deform their surface: Given a plate with one surface sample of 0.01 m, scale 5 and phase 1 Then the ghost prism stays and the deformed surface lies 0.05 m above the mid plane
  - Mode labels carry frequency and critical load factor: Given a mode overlay with f = 2.5 Hz and alpha_cr = 3.42 Then the English label is "Mode 1 · f = 2.5 Hz · αcr = 3.42 · ×10" And the German label is "Eigenform 1 · f = 2,5 Hz · αcr = 3,42 · ×10"
  - Shell eigenmode samples use physical surface coordinates: Given a 3 x 3 shell grid with 2 m between columns and 1 m between rows Then the surface coordinates are 0, 2, 4 m along the columns and 0, 1, 2 m along the rows And a sample at 2 m, 1 m of 0.1 m lifts the centre node by 0.1 m
  - Scene tracks drive the viewport and unselected tracks rest on their first keyframe: Given a scene with a mode track whose keyframes reveal the layers "mode-1" and "mode-2" Then at rest only "mode-1" is drawn and at track value 2 only "mode-2" is drawn
  - The toolbar offers fields, tracks and a localized readout: Given a scene with the track "fire" at 1800 s Then the English readout is "Fire duration: 30 min" and the German one "Branddauer: 30 min"
