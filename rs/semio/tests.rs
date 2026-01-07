// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 semio contributors

use super::*;

// #region Utility Tests

#[test]
fn test_guid_generation() {
    let g1 = guid();
    let g2 = guid();
    assert_ne!(g1, g2);
    assert_eq!(g1.len(), 36);
}

#[test]
fn test_normalize() {
    assert_eq!(normalize(3.14159, 2), 3.14);
    assert_eq!(normalize(3.145, 2), 3.15);
    assert_eq!(normalize(3.144, 2), 3.14);
}

#[test]
fn test_jaccard() {
    let a: std::collections::HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let b: std::collections::HashSet<_> = vec![2, 3, 4].into_iter().collect();
    let j = jaccard(&a, &b);
    assert!((j - 0.5).abs() < 0.001);
    
    let empty: std::collections::HashSet<i32> = std::collections::HashSet::new();
    assert_eq!(jaccard(&empty, &empty), 1.0);
}

#[test]
fn test_generate_unique_name() {
    let existing = vec!["Test".to_string(), "Test (1)".to_string()];
    assert_eq!(generate_unique_name("New", &existing), "New");
    assert_eq!(generate_unique_name("Test", &existing), "Test (2)");
}

// #endregion Utility Tests

// #region Model Tests

#[test]
fn test_vector_creation() {
    let v = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vector_defaults() {
    assert_eq!(Vector::zero(), Vector::new(0.0, 0.0, 0.0));
    assert_eq!(Vector::unit_x(), Vector::new(1.0, 0.0, 0.0));
    assert_eq!(Vector::unit_y(), Vector::new(0.0, 1.0, 0.0));
    assert_eq!(Vector::unit_z(), Vector::new(0.0, 0.0, 1.0));
}

#[test]
fn test_plane_default() {
    let p = Plane::default();
    assert_eq!(p.origin, Vector::zero());
    assert_eq!(p.x_axis, Vector::unit_x());
    assert_eq!(p.y_axis, Vector::unit_y());
}

#[test]
fn test_coord_creation() {
    let c = Coord::new(1.5, 2.5);
    assert_eq!(c.u, 1.5);
    assert_eq!(c.v, 2.5);
}

// #endregion Model Tests

// #region Serialization Tests

#[test]
fn test_kit_serialization_roundtrip() {
    let kit = Kit {
        guid: guid(),
        name: "Test Kit".to_string(),
        version: Some("1.0.0".to_string()),
        description: Some("A test kit".to_string()),
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: None,
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };

    let json = serialize_kit(&kit).unwrap();
    let deserialized = deserialize_kit(&json).unwrap();
    
    assert_eq!(kit.guid, deserialized.guid);
    assert_eq!(kit.name, deserialized.name);
    assert_eq!(kit.version, deserialized.version);
    assert_eq!(kit.description, deserialized.description);
}

#[test]
fn test_design_serialization() {
    let design = Design {
        guid: guid(),
        name: "Test Design".to_string(),
        parent: None,
        description: None,
        icon: None,
        image: None,
        folder: None,
        unit: None,
        is_abstract: None,
        can_scale: None,
        can_mirror: None,
        concepts: None,
        authors: None,
        props: None,
        pieces: Some(vec![Piece {
            guid: guid(),
            name: Some("Piece 1".to_string()),
            type_ref: None,
            design: None,
            plane: Some(Plane::default()),
            center: Some(Coord::new(0.0, 0.0)),
            scale: None,
            mirror_plane: None,
            is_hidden: None,
            is_locked: None,
            color: None,
            description: None,
            props: None,
            attributes: None,
        }]),
        connections: None,
        layers: None,
        groups: None,
        stats: None,
        active_layer: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };

    let json = serialize_design(&design).unwrap();
    let deserialized = deserialize_design(&json).unwrap();
    
    assert_eq!(design.guid, deserialized.guid);
    assert_eq!(design.name, deserialized.name);
    assert!(deserialized.pieces.is_some());
    assert_eq!(deserialized.pieces.as_ref().unwrap().len(), 1);
}

// #endregion Serialization Tests

// #region Diff Tests

#[test]
fn test_attribute_diff() {
    let before = Attribute {
        guid: "attr-1".to_string(),
        key: "color".to_string(),
        value: Some("red".to_string()),
        definition: None,
    };
    
    let after = Attribute {
        guid: "attr-1".to_string(),
        key: "color".to_string(),
        value: Some("blue".to_string()),
        definition: None,
    };
    
    let diff = get_attribute_diff(&before, &after);
    assert!(diff.key.is_none());
    assert_eq!(diff.value, Some("blue".to_string()));
    assert!(diff.definition.is_none());
}

#[test]
fn test_apply_attribute_diff() {
    let mut attr = Attribute {
        guid: "attr-1".to_string(),
        key: "color".to_string(),
        value: Some("red".to_string()),
        definition: None,
    };
    
    let diff = AttributeDiff {
        key: None,
        value: Some("green".to_string()),
        definition: Some("A color attribute".to_string()),
    };
    
    apply_attribute_diff(&mut attr, &diff);
    
    assert_eq!(attr.value, Some("green".to_string()));
    assert_eq!(attr.definition, Some("A color attribute".to_string()));
}

#[test]
fn test_attributes_diff() {
    let before = Some(vec![
        Attribute {
            guid: "attr-1".to_string(),
            key: "a".to_string(),
            value: Some("1".to_string()),
            definition: None,
        },
        Attribute {
            guid: "attr-2".to_string(),
            key: "b".to_string(),
            value: Some("2".to_string()),
            definition: None,
        },
    ]);
    
    let after = Some(vec![
        Attribute {
            guid: "attr-1".to_string(),
            key: "a".to_string(),
            value: Some("modified".to_string()),
            definition: None,
        },
        Attribute {
            guid: "attr-3".to_string(),
            key: "c".to_string(),
            value: Some("3".to_string()),
            definition: None,
        },
    ]);
    
    let diff = get_attributes_diff(&before, &after);
    
    assert!(diff.added.is_some());
    assert_eq!(diff.added.as_ref().unwrap().len(), 1);
    assert_eq!(diff.added.as_ref().unwrap()[0].guid, "attr-3");
    
    assert!(diff.removed.is_some());
    assert_eq!(diff.removed.as_ref().unwrap().len(), 1);
    assert_eq!(diff.removed.as_ref().unwrap()[0].guid, "attr-2");
    
    assert!(diff.updated.is_some());
    assert_eq!(diff.updated.as_ref().unwrap().len(), 1);
    assert_eq!(diff.updated.as_ref().unwrap()[0].attribute.guid, "attr-1");
}

#[test]
fn test_kit_diff() {
    let before = Kit {
        guid: "kit-1".to_string(),
        name: "Original".to_string(),
        version: Some("1.0.0".to_string()),
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: None,
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let after = Kit {
        guid: "kit-1".to_string(),
        name: "Modified".to_string(),
        version: Some("2.0.0".to_string()),
        description: Some("A description".to_string()),
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: None,
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let diff = get_kit_diff(&before, &after);
    
    assert_eq!(diff.name, Some("Modified".to_string()));
    assert_eq!(diff.version, Some("2.0.0".to_string()));
    assert_eq!(diff.description, Some("A description".to_string()));
}

#[test]
fn test_apply_kit_diff() {
    let mut kit = Kit {
        guid: "kit-1".to_string(),
        name: "Original".to_string(),
        version: Some("1.0.0".to_string()),
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: None,
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let diff = KitDiff {
        name: Some("Updated".to_string()),
        version: Some("2.0.0".to_string()),
        description: Some("New description".to_string()),
        ..Default::default()
    };
    
    apply_kit_diff(&mut kit, &diff);
    
    assert_eq!(kit.name, "Updated");
    assert_eq!(kit.version, Some("2.0.0".to_string()));
    assert_eq!(kit.description, Some("New description".to_string()));
}

// #endregion Diff Tests

// #region Validation Tests

#[test]
fn test_guid_uniqueness_validation() {
    let kit = Kit {
        guid: "kit-1".to_string(),
        name: "Test Kit".to_string(),
        version: None,
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: Some(vec![
            Type {
                guid: "type-1".to_string(),
                name: "Type A".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
            Type {
                guid: "type-1".to_string(),
                name: "Type B".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
        ]),
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let result = validate_kit(&kit, None);
    assert!(result.has_errors());
    assert!(result.problems.iter().any(|p| p.constraint_id == "guid-unique"));
}

#[test]
fn test_type_name_uniqueness_validation() {
    let kit = Kit {
        guid: "kit-1".to_string(),
        name: "Test Kit".to_string(),
        version: None,
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: Some(vec![
            Type {
                guid: "type-1".to_string(),
                name: "SameName".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
            Type {
                guid: "type-2".to_string(),
                name: "SameName".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
        ]),
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let result = validate_kit(&kit, None);
    assert!(result.has_errors());
    assert!(result.problems.iter().any(|p| p.constraint_id == "type-name-unique"));
}

#[test]
fn test_valid_kit_passes_validation() {
    let kit = Kit {
        guid: "kit-1".to_string(),
        name: "Test Kit".to_string(),
        version: None,
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: Some(vec![
            Type {
                guid: "type-1".to_string(),
                name: "Type A".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
            Type {
                guid: "type-2".to_string(),
                name: "Type B".to_string(),
                parent: None,
                description: None,
                icon: None,
                image: None,
                folder: None,
                unit: None,
                stock: None,
                is_abstract: None,
                virtual_type: None,
                location: None,
                concepts: None,
                authors: None,
                props: None,
                models: None,
                connectors: None,
                attributes: None,
                created_at: None,
                updated_at: None,
            },
        ]),
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let result = validate_kit(&kit, None);
    assert!(!result.has_errors());
}

// #endregion Validation Tests

// #region IO Tests

#[test]
fn test_supported_model_extensions() {
    assert!(is_supported_model_extension("gltf"));
    assert!(is_supported_model_extension("GLB"));
    assert!(is_supported_model_extension(".obj"));
    assert!(!is_supported_model_extension("txt"));
    assert!(!is_supported_model_extension("png"));
}

#[test]
fn test_validate_model_file() {
    assert!(validate_model_file("model.gltf"));
    assert!(validate_model_file("path/to/model.glb"));
    assert!(!validate_model_file("image.png"));
    assert!(!validate_model_file("no_extension"));
}

// #endregion IO Tests

// #region Finder Tests

#[test]
fn test_find_type_in_kit() {
    let kit = Kit {
        guid: "kit-1".to_string(),
        name: "Test".to_string(),
        version: None,
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: Some(vec![Type {
            guid: "type-1".to_string(),
            name: "Found Type".to_string(),
            parent: None,
            description: None,
            icon: None,
            image: None,
            folder: None,
            unit: None,
            stock: None,
            is_abstract: None,
            virtual_type: None,
            location: None,
            concepts: None,
            authors: None,
            props: None,
            models: None,
            connectors: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        }]),
        designs: None,
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let found = find_type_in_kit(&kit, "type-1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Found Type");
    
    let not_found = find_type_in_kit(&kit, "nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn test_find_design_in_kit() {
    let kit = Kit {
        guid: "kit-1".to_string(),
        name: "Test".to_string(),
        version: None,
        description: None,
        icon: None,
        image: None,
        preview: None,
        remote: None,
        homepage: None,
        license: None,
        concepts: None,
        tags: None,
        types: None,
        designs: Some(vec![Design {
            guid: "design-1".to_string(),
            name: "Found Design".to_string(),
            parent: None,
            description: None,
            icon: None,
            image: None,
            folder: None,
            unit: None,
            is_abstract: None,
            can_scale: None,
            can_mirror: None,
            concepts: None,
            authors: None,
            props: None,
            pieces: None,
            connections: None,
            layers: None,
            groups: None,
            stats: None,
            active_layer: None,
            attributes: None,
            created_at: None,
            updated_at: None,
        }]),
        ports: None,
        qualities: None,
        files: None,
        folders: None,
        authors: None,
        attributes: None,
        created_at: None,
        updated_at: None,
    };
    
    let found = find_design_in_kit(&kit, "design-1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Found Design");
}

#[test]
fn test_is_fixed_piece() {
    let fixed_piece = Piece {
        guid: "piece-1".to_string(),
        name: None,
        type_ref: None,
        design: None,
        plane: Some(Plane::default()),
        center: None,
        scale: None,
        mirror_plane: None,
        is_hidden: None,
        is_locked: None,
        color: None,
        description: None,
        props: None,
        attributes: None,
    };
    
    let linked_piece = Piece {
        guid: "piece-2".to_string(),
        name: None,
        type_ref: None,
        design: None,
        plane: None,
        center: None,
        scale: None,
        mirror_plane: None,
        is_hidden: None,
        is_locked: None,
        color: None,
        description: None,
        props: None,
        attributes: None,
    };
    
    assert!(is_fixed_piece(&fixed_piece));
    assert!(!is_fixed_piece(&linked_piece));
}

// #endregion Finder Tests
