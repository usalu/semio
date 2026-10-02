# Original DWG Metadata Constructors

Exact authored pre-cut source retained for field-type review. Portable key/optional/label fixtures were authored independently from the invariant census before this constructor transformation.

```rust
fn dwg_xrecord_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("string".into(), 0),
                    ("real".into(), 1),
                    ("boolean".into(), 2),
                    ("integer8".into(), 3),
                    ("integer16".into(), 4),
                    ("integer32".into(), 5),
                    ("integer64".into(), 6),
                    ("point3d".into(), 7),
                    ("binary".into(), 8),
                    ("handle".into(), 9),
                    ("objectId".into(), 10),
                ]),
            ),
            dsl::FieldSpec::new(1, "group_code", dsl::Shape::Int),
            dsl::FieldSpec::new(2, "string_value", dsl::Shape::Text).optional(),
            dsl::FieldSpec::new(3, "real_value", dsl::Shape::Float).optional(),
            dsl::FieldSpec::new(4, "boolean_value", dsl::Shape::Bool).optional(),
            dsl::FieldSpec::new(5, "integer_value", dsl::Shape::Int).optional(),
            dsl::FieldSpec::new(6, "point_value", dsl::Shape::Tuple(Box::new(dsl::Shape::Float), Some(3))).optional(),
            dsl::FieldSpec::new(7, "binary_octets", dsl::Shape::Bytes64).optional(),
            dsl::FieldSpec::new(8, "handle_value", dsl::Shape::UInt).optional(),
        ],
    )
}
```

```rust
fn dwg_table_control_entry_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "has_handle", dsl::Shape::Bool), dsl::FieldSpec::new(1, "handle", dsl::Shape::UInt).optional()])
}
```

```rust
fn table_control_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                1,
                "kind",
                dsl::Shape::Enum(vec![
                    ("block".into(), 0),
                    ("layer".into(), 1),
                    ("textStyle".into(), 2),
                    ("linetype".into(), 3),
                    ("view".into(), 4),
                    ("ucs".into(), 5),
                    ("viewport".into(), 6),
                    ("registeredApplication".into(), 7),
                    ("dimensionStyle".into(), 8),
                ]),
            ),
            dsl::FieldSpec::new(2, "entries", <DwgTableControlEntries as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "block", <DwgBlockTableControl as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "linetype", <DwgLinetypeTableControl as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "dimensionStyle", <DwgDimensionStyleTableControl as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn dwg_complex_color_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![("none".into(), 0), ("byLayer".into(), 1), ("byBlock".into(), 2), ("byColor".into(), 3), ("byAci".into(), 4), ("byPen".into(), 5), ("foreground".into(), 6), ("layerOff".into(), 7), ("layerFrozen".into(), 8)]),
            ),
            dsl::FieldSpec::new(1, "red", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "green", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "blue", <u8 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "index", <u16 as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn table_record_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                1,
                "kind",
                dsl::Shape::Enum(vec![("registeredApplication".into(), 0), ("textStyle".into(), 1), ("layer".into(), 2), ("linetype".into(), 3), ("blockHeader".into(), 4), ("viewport".into(), 5), ("dimensionStyle".into(), 6)]),
            ),
            dsl::FieldSpec::new(2, "registeredApplication", <DwgRegisteredApplicationTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "textStyle", <DwgTextStyleTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "layer", <DwgLayerTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "linetype", <DwgLinetypeTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "blockHeader", <DwgBlockHeaderTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "viewport", <DwgViewportTableRecord as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "dimensionStyle", <DwgDimensionStyleTableRecord as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn dwg_evaluation_variant_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "kind", dsl::Shape::Enum(vec![("integer32".into(), 0)])), dsl::FieldSpec::new(1, "integer32", <i32 as dsl::DslField>::shape()).optional()])
}
```

```rust
fn dwg_evaluation_expression_value_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![("empty".into(), 0), ("double".into(), 1), ("pointGroup10".into(), 2), ("pointGroup11".into(), 3), ("string".into(), 4), ("integer32".into(), 5), ("objectReference".into(), 6), ("integer16".into(), 7)]),
            ),
            dsl::FieldSpec::new(1, "double", <f64 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "point_group_10", <Vec<f64> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "point_group_11", <Vec<f64> as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "string", <String as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "integer32", <i32 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "object_reference", <u64 as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "integer16", <i16 as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn dwg_visual_style_property_spec<T: dsl::DslField>() -> dsl::RecordSpec {
    dsl::RecordSpec::new(None, dsl::RecordLayout::Inline, vec![dsl::FieldSpec::new(0, "value", T::shape()), dsl::FieldSpec::new(1, "operation", <DwgVisualStylePropertyOperation as dsl::DslField>::shape())])
}
```

```rust
fn dwg_constraint_node_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("constrainedImplicitPoint".into(), 0),
                    ("pointCurveConstraint".into(), 1),
                    ("constrainedBoundedLine".into(), 2),
                    ("pointCoincidenceConstraint".into(), 3),
                    ("distanceConstraint".into(), 4),
                    ("perpendicularConstraint".into(), 5),
                    ("horizontalConstraint".into(), 6),
                    ("parallelConstraint".into(), 7),
                    ("midPointConstraint".into(), 8),
                    ("equalLengthConstraint".into(), 9),
                    ("colinearConstraint".into(), 10),
                    ("constrainedDatumLine".into(), 11),
                    ("fixedConstraint".into(), 12),
                    ("verticalConstraint".into(), 13),
                ]),
            ),
            dsl::FieldSpec::new(1, "constrained_implicit_point", <DwgConstrainedImplicitPoint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "geometric_constraint", <DwgGeometricConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "constrained_bounded_line", <DwgConstrainedBoundedLine as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "distance_constraint", <DwgDistanceConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "axis_constraint", <DwgAxisConstraint as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "constrained_datum_line", <DwgConstrainedDatumLine as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn dwg_entity_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("line".into(), 0),
                    ("arc".into(), 1),
                    ("lwPolyline".into(), 2),
                    ("blockBegin".into(), 3),
                    ("blockEnd".into(), 4),
                    ("insert".into(), 5),
                    ("dimensionLinear".into(), 6),
                    ("viewport".into(), 7),
                    ("point".into(), 8),
                    ("circle".into(), 9),
                    ("ellipse".into(), 10),
                    ("text".into(), 11),
                    ("spline".into(), 12),
                    ("face3d".into(), 13),
                    ("polyline3d".into(), 14),
                    ("polyfaceMesh".into(), 15),
                    ("vertex".into(), 16),
                    ("polyfaceFace".into(), 17),
                    ("sequenceEnd".into(), 18),
                ]),
            ),
            dsl::FieldSpec::new(1, "line", <DwgLineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "arc", <DwgArcEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "lw_polyline", <DwgLwPolylineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "block_begin", <DwgBlockBeginEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "block_end", <DwgBlockEndEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "insert", <DwgInsertEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "dimension_linear", <DwgLinearDimensionEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "viewport", <DwgViewportEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(9, "point", <DwgPointEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(10, "circle", <DwgCircleEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(11, "ellipse", <DwgEllipseEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(12, "text", <DwgTextEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(13, "spline", <DwgSplineEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(14, "face3d", <DwgFace3dEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(15, "polyline3d", <DwgPolyline3dEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(16, "polyface_mesh", <DwgPolyfaceMeshEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(17, "vertex", <DwgVertexEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(18, "polyface_face", <DwgPolyfaceFaceEntity as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(19, "sequence_end", <DwgSequenceEndEntity as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

```rust
fn dwg_logical_object_body_spec() -> dsl::RecordSpec {
    dsl::RecordSpec::new(
        None,
        dsl::RecordLayout::Inline,
        vec![
            dsl::FieldSpec::new(
                0,
                "kind",
                dsl::Shape::Enum(vec![
                    ("dictionary".into(), 0),
                    ("tableControl".into(), 1),
                    ("tableRecord".into(), 2),
                    ("xrecord".into(), 3),
                    ("entity".into(), 4),
                    ("associativeDependency".into(), 5),
                    ("associativeValueDependency".into(), 6),
                    ("associativeGeometryDependency".into(), 7),
                    ("blockGripLocationComponent".into(), 8),
                    ("dynamicBlockProxyNode".into(), 9),
                    ("associativeVariable".into(), 10),
                    ("associativeDimensionDependencyBody".into(), 11),
                    ("visualStyle".into(), 12),
                    ("blockParameterDependencyBody".into(), 13),
                    ("blockRepresentationData".into(), 14),
                    ("dynamicBlockPurgePreventer".into(), 15),
                    ("evaluationGraph".into(), 16),
                    ("blockFlipParameter".into(), 17),
                    ("blockVisibilityParameter".into(), 18),
                    ("placeholder".into(), 19),
                    ("dictionaryVariable".into(), 20),
                    ("annotationScale".into(), 21),
                    ("sortEntitiesTable".into(), 22),
                    ("tableStyle".into(), 23),
                    ("mlineStyle".into(), 24),
                    ("mLeaderStyle".into(), 25),
                    ("material".into(), 26),
                    ("blockMoveAction".into(), 27),
                    ("assocNetwork".into(), 28),
                    ("assoc2dConstraintGroup".into(), 29),
                    ("blockLinearParameter".into(), 30),
                    ("blockLinearGrip".into(), 31),
                    ("blockFlipGrip".into(), 32),
                    ("blockVisibilityGrip".into(), 33),
                    ("blockAlignmentParameter".into(), 34),
                    ("blockAlignmentGrip".into(), 35),
                    ("blockStretchAction".into(), 36),
                    ("blockScaleAction".into(), 37),
                    ("blockFlipAction".into(), 38),
                    ("blockBasePointParameter".into(), 39),
                    ("blockVerticalConstraintParameter".into(), 40),
                    ("blockHorizontalConstraintParameter".into(), 41),
                    ("layout".into(), 42),
                ]),
            ),
            dsl::FieldSpec::new(1, "dictionary", <DwgDictionaryBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(2, "table_control", <DwgTableControlBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(3, "table_record", <DwgTableRecordBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(4, "xrecord", <DwgXRecordBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(5, "entity", <DwgEntityBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(6, "associative_dependency", <DwgAssociativeDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(7, "associative_value_dependency", <DwgAssociativeValueDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(8, "associative_geometry_dependency", <DwgAssociativeGeometryDependency as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(9, "block_grip_location_component", <DwgBlockGripLocationComponent as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(10, "dynamic_block_proxy_node", <DwgDynamicBlockProxyNode as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(11, "associative_variable", <DwgAssociativeVariable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(12, "associative_dimension_dependency_body", <DwgAssociativeDimensionDependencyBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(13, "visual_style", <DwgVisualStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(14, "block_parameter_dependency_body", <DwgBlockParameterDependencyBody as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(15, "block_representation_data", <DwgBlockRepresentationData as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(16, "dynamic_block_purge_preventer", <DwgDynamicBlockPurgePreventer as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(17, "evaluation_graph", <DwgEvaluationGraph as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(18, "block_flip_parameter", <DwgBlockFlipParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(19, "block_visibility_parameter", <DwgBlockVisibilityParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(20, "placeholder", <DwgPlaceholder as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(21, "dictionary_variable", <DwgDictionaryVariable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(22, "annotation_scale", <DwgAnnotationScale as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(23, "sort_entities_table", <DwgSortEntitiesTable as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(24, "table_style", <DwgTableStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(25, "mline_style", <DwgMlineStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(26, "m_leader_style", <DwgMLeaderStyle as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(27, "material", <DwgMaterial as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(28, "block_move_action", <DwgBlockMoveAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(29, "assoc_network", <DwgAssocNetwork as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(30, "assoc_2d_constraint_group", <DwgAssoc2dConstraintGroup as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(31, "block_linear_parameter", <DwgBlockLinearParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(32, "block_linear_grip", <DwgBlockLinearGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(33, "block_flip_grip", <DwgBlockFlipGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(34, "block_visibility_grip", <DwgBlockVisibilityGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(35, "block_alignment_parameter", <DwgBlockAlignmentParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(36, "block_alignment_grip", <DwgBlockAlignmentGrip as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(37, "block_stretch_action", <DwgBlockStretchAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(38, "block_scale_action", <DwgBlockScaleAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(39, "block_flip_action", <DwgBlockFlipAction as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(40, "block_base_point_parameter", <DwgBlockBasePointParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(41, "block_vertical_constraint_parameter", <DwgBlockLinearConstraintParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(42, "block_horizontal_constraint_parameter", <DwgBlockLinearConstraintParameter as dsl::DslField>::shape()).optional(),
            dsl::FieldSpec::new(43, "layout", <DwgLayout as dsl::DslField>::shape()).optional(),
        ],
    )
}
```

