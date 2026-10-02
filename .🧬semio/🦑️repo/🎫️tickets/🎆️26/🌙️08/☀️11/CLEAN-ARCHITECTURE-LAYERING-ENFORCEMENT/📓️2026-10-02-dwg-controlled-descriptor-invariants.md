# DWG Controlled Metadata Descriptor Invariants

Read-only source census; no tests/compiler run. Companion: 📓️2026-10-02-dwg-controlled-record-spec-frontier.md. Root delegated execution to HighNeutral; NativeHigh remains sole compiler queue.

Single authored descriptor should retain field ID/key/shape/optional plus enum label/value order, while ordinary and controlled constructors interpret the same descriptor separately. A callback that merely returns an ordinary preallocated shape is not a controlled descriptor. Typed child callbacks must take &mut C and call T::shape_controlled(control), including generic visual-style T; lazy Record producer callbacks must remain lazy, not unfold child records.

Actual authored roster:

## dwg_xrecord_value_spec (line 108)
Fields: 0:kind, 1:group_code, 2:string_value, 3:real_value, 4:boolean_value, 5:integer_value, 6:point_value, 7:binary_octets, 8:handle_value.
Optional calls: 7. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: string:0, real:1, boolean:2, integer8:3, integer16:4, integer32:5, integer64:6, point3d:7, binary:8, handle:9, objectId:10.

## dwg_table_control_entry_spec (line 333)
Fields: 0:has_handle, 1:handle.
Optional calls: 1. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: none.

## table_control_body_spec (line 430)
Fields: 1:kind, 2:entries, 3:block, 4:linetype, 5:dimensionStyle.
Optional calls: 4. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: block:0, layer:1, textStyle:2, linetype:3, view:4, ucs:5, viewport:6, registeredApplication:7, dimensionStyle:8.

## dwg_complex_color_value_spec (line 553)
Fields: 0:kind, 1:red, 2:green, 3:blue, 4:index.
Optional calls: 4. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: none:0, byLayer:1, byBlock:2, byColor:3, byAci:4, byPen:5, foreground:6, layerOff:7, layerFrozen:8.

## table_record_body_spec (line 895)
Fields: 1:kind, 2:registeredApplication, 3:textStyle, 4:layer, 5:linetype, 6:blockHeader, 7:viewport, 8:dimensionStyle.
Optional calls: 7. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: registeredApplication:0, textStyle:1, layer:2, linetype:3, blockHeader:4, viewport:5, dimensionStyle:6.

## dwg_evaluation_variant_spec (line 1535)
Fields: 0:kind, 1:integer32.
Optional calls: 1. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: integer32:0.

## dwg_evaluation_expression_value_spec (line 1595)
Fields: 0:kind, 1:double, 2:point_group_10, 3:point_group_11, 4:string, 5:integer32, 6:object_reference, 7:integer16.
Optional calls: 7. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: empty:0, double:1, pointGroup10:2, pointGroup11:3, string:4, integer32:5, objectReference:6, integer16:7.

## dwg_visual_style_property_spec (line 1752)
Fields: 0:value, 1:operation.
Optional calls: 0. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: none.

## dwg_constraint_node_spec (line 2785)
Fields: 0:kind, 1:constrained_implicit_point, 2:geometric_constraint, 3:constrained_bounded_line, 4:distance_constraint, 5:axis_constraint, 6:constrained_datum_line.
Optional calls: 6. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: constrainedImplicitPoint:0, pointCurveConstraint:1, constrainedBoundedLine:2, pointCoincidenceConstraint:3, distanceConstraint:4, perpendicularConstraint:5, horizontalConstraint:6, parallelConstraint:7, midPointConstraint:8, equalLengthConstraint:9, colinearConstraint:10, constrainedDatumLine:11, fixedConstraint:12, verticalConstraint:13.

## dwg_entity_body_spec (line 2885)
Fields: 0:kind, 1:line, 2:arc, 3:lw_polyline, 4:block_begin, 5:block_end, 6:insert, 7:dimension_linear, 8:viewport, 9:point, 10:circle, 11:ellipse, 12:text, 13:spline, 14:face3d, 15:polyline3d, 16:polyface_mesh, 17:vertex, 18:polyface_face, 19:sequence_end.
Optional calls: 19. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: line:0, arc:1, lwPolyline:2, blockBegin:3, blockEnd:4, insert:5, dimensionLinear:6, viewport:7, point:8, circle:9, ellipse:10, text:11, spline:12, face3d:13, polyline3d:14, polyfaceMesh:15, vertex:16, polyfaceFace:17, sequenceEnd:18.

## dwg_logical_object_body_spec (line 3102)
Fields: 0:kind, 1:dictionary, 2:table_control, 3:table_record, 4:xrecord, 5:entity, 6:associative_dependency, 7:associative_value_dependency, 8:associative_geometry_dependency, 9:block_grip_location_component, 10:dynamic_block_proxy_node, 11:associative_variable, 12:associative_dimension_dependency_body, 13:visual_style, 14:block_parameter_dependency_body, 15:block_representation_data, 16:dynamic_block_purge_preventer, 17:evaluation_graph, 18:block_flip_parameter, 19:block_visibility_parameter, 20:placeholder, 21:dictionary_variable, 22:annotation_scale, 23:sort_entities_table, 24:table_style, 25:mline_style, 26:m_leader_style, 27:material, 28:block_move_action, 29:assoc_network, 30:assoc_2d_constraint_group, 31:block_linear_parameter, 32:block_linear_grip, 33:block_flip_grip, 34:block_visibility_grip, 35:block_alignment_parameter, 36:block_alignment_grip, 37:block_stretch_action, 38:block_scale_action, 39:block_flip_action, 40:block_base_point_parameter, 41:block_vertical_constraint_parameter, 42:block_horizontal_constraint_parameter, 43:layout.
Optional calls: 43. All records keyword None and layout Inline; position/flatten/defines/call-name defaults retained.
Enum labels: dictionary:0, tableControl:1, tableRecord:2, xrecord:3, entity:4, associativeDependency:5, associativeValueDependency:6, associativeGeometryDependency:7, blockGripLocationComponent:8, dynamicBlockProxyNode:9, associativeVariable:10, associativeDimensionDependencyBody:11, visualStyle:12, blockParameterDependencyBody:13, blockRepresentationData:14, dynamicBlockPurgePreventer:15, evaluationGraph:16, blockFlipParameter:17, blockVisibilityParameter:18, placeholder:19, dictionaryVariable:20, annotationScale:21, sortEntitiesTable:22, tableStyle:23, mlineStyle:24, mLeaderStyle:25, material:26, blockMoveAction:27, assocNetwork:28, assoc2dConstraintGroup:29, blockLinearParameter:30, blockLinearGrip:31, blockFlipGrip:32, blockVisibilityGrip:33, blockAlignmentParameter:34, blockAlignmentGrip:35, blockStretchAction:36, blockScaleAction:37, blockFlipAction:38, blockBasePointParameter:39, blockVerticalConstraintParameter:40, blockHorizontalConstraintParameter:41, layout:42.

## Existing executable ownership
Rust package 📜️script.ts delegates to canonical runArtifactRustPackageMain with package semio-s-artifact-stdio-dwg. Existing AC1024 IO unit owner 🚪️io/🧪️tests/🔬️unit/🦀️.rs retains logical classes/sections roundtrips298/309, lossless system roundtrip355, committed facets448, no shadow state469, grammar529/547/560, protocol walk574 and fixture honesty605. These retain original artifact behavior; they are not an independent exact controlled-metadata descriptor oracle. Use authored language-neutral descriptor goldens plus Serde JSON output parity for all11 records, with bounded recursive projection resolving lazy children only under controller authority. Native ceiling/cancellation tests must remain distinct from metadata-value equality. Do not count unrelated controlled_expression_handles model fields as existing control hooks.