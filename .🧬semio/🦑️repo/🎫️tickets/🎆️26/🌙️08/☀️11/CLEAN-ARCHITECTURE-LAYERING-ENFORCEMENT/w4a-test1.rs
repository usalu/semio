    #[test]
    fn media_types_compatible_covers_direct_any_convert_and_reject() {
        let brep = MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep };
        let mesh_form = MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh };
        let any_3d = MediaType { class: MediaClass::ThreeD, form: MediaForm::Any };
        let vector = MediaType { class: MediaClass::TwoD, form: MediaForm::Vector };
        let raster = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
        let text = MediaType { class: MediaClass::Text, form: MediaForm::Document };

        assert_eq!(media_types_compatible(&brep, &brep), MediaCompat::Direct);
        assert_eq!(media_types_compatible(&brep, &any_3d), MediaCompat::Direct, "Any on the accepting side takes anything within the class");
        assert!(matches!(media_types_compatible(&brep, &mesh_form), MediaCompat::Convert { from: MediaForm::Brep, to: MediaForm::Mesh }));
        assert!(matches!(media_types_compatible(&vector, &raster), MediaCompat::Convert { from: MediaForm::Vector, to: MediaForm::Raster }));
        assert_eq!(media_types_compatible(&mesh_form, &brep), MediaCompat::Reject, "mesh->brep has no registered conversion");
        assert_eq!(media_types_compatible(&brep, &text), MediaCompat::Reject, "class mismatch always rejects");
    }
