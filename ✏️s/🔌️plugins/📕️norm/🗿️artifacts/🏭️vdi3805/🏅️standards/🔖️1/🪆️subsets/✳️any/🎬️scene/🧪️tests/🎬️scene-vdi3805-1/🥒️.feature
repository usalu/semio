@capability-vdi3805-1-evaluate
@oracle-vdi3805-1-compliance-independent
@comparison-numerical-v1
Feature: VDI 3805 structural scene
  The manufacturer dataset and its check report project to a deterministic, locale-free 3D scene, a set of 2D diagrams and drag handles.

  Further specification assertions, asserted by the Rust unit tests of the crate and the independent Python oracle:
  - products are drawn at the size of their geometry: Given the conforming valve dataset When the scene is projected Then entity "VLV-50-001" is a box of the scaled bounding box of "geom-valve-50"
  - a radiator without geometry is drawn from its extent: Given the conforming Blatt 3 dataset When the scene is projected Then entity "RAD-600" is a box of 0.6 by 0.1 by 0.6 metres
  - the radiator output follows EN 442: Given the conforming Blatt 3 dataset When the diagrams are built Then diagram "output:RAD-600" equals 1200 W times the power 1.3 of the excess temperature over 49.83 K And a numpy oracle reproduces the curve
  - the valve pressure drop follows the square law: Given the conforming valve dataset When the diagrams are built Then diagram "drop:VLV-50-001" equals the square of the flow over kvs and is one bar at the flow kvs
  - face grips write the bounding box: Given the conforming valve dataset When the width grip of "VLV-50-001" is dragged Then one write sets "geometry[id=geom-valve-50].bbox.maxX" And the resting grip reads the stored extent back
  - curve points are draggable: Given the conforming valve dataset When the curve diagram of "VLV-50-001" is built Then every point carries a marker bound to "curves[id=curve-kvs].points[i].y"
  - the golden scene is stable: Given the conforming valve dataset When the scene is projected twice Then both canonical JSON texts are equal and equal the committed golden
