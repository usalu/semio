mod tests {
    use super::*;
    use crate::editor::animate::engine::animation::animation::Animations;
    use crate::editor::animate::engine::geometry::geometry::circle;
    use crate::editor::animate::engine::scene::sobject::VSobject;
    use crate::editor::animate::engine::text::color::Color;

    #[test]
    fn catalog_stubs_compile_and_apply() {
        let mut map: HashMap<u64, Sobjects> = HashMap::new();
        let v = VSobject::new();
        let id = v.id();
        map.insert(id, v.into());
        let v2 = circle(Point::new(2.0, 0.0), 0.5, Color::WHITE, None, 1.0);
        let id2 = v2.id();
        map.insert(id2, v2.into());

        let stubs: Vec<Animations> = vec![
            Uncreate::new(id, 1.0).into(),
            Write::new(id, 1.0).into(),
            DrawBorderThenFill::new(id, 1.0).into(),
            FadeTransform::new(id, 1.0).into(),
            ReplacementTransform::new(id, 1.0).into(),
            TransformFromCopy::new(id, 1.0).into(),
            MoveToTarget::new(id, 1.0).into(),
            Restore::new(id, 1.0).into(),
            Indicate::new(id, 1.0).into(),
            Flash::new(id, 1.0).into(),
            Circumscribe::new(id, 1.0).into(),
            GrowFromCenter::new(id, 1.0).into(),
            GrowFromPoint::new(id, 1.0).into(),
            ShrinkToCenter::new(id, 1.0).into(),
            SpinInFromNothing::new(id, 1.0).into(),
            ChangeDecimalToValue::new(id, 1.0).into(),
            Broadcast::new(id, 1.0).into(),
            ApplyWave::new(id, 1.0).into(),
            Wiggle::new(id, 1.0).into(),
            CyclicReplace::new(id, 1.0).into(),
            Swap::new(id, id2, 1.0).into(),
            TransformMatchingShapes::new(id, 1.0).into(),
            Homotopy::new(id, 1.0).into(),
            ShowPassingFlash::new(id, 1.0).into(),
            SpiralIn::new(id, 1.0).into(),
            Rotating::new(id, 1.0).into(),
        ];
        for mut anim in stubs {
            anim.apply(&mut map, 0.5);
            assert!(!anim.get_all_mobjects().is_empty());
        }

        with_vsobject(&mut map, id, |v| {
            assert!(v.point_ratio > 0.0);
            assert!(v.opacity() > 0.0);
        });
        with_vsobject(&mut map, id2, |v| {
            assert!(v.center().x() > 0.5);
        });
    }

    #[test]
    fn write_reveals_point_ratio() {
        let mut map: HashMap<u64, Sobjects> = HashMap::new();
        let v = circle(Point::ZERO, 1.0, Color::WHITE, None, 1.0);
        let id = v.id();
        map.insert(id, v.into());
        let mut write = Write::new(id, 1.0);
        write.apply(&mut map, 0.5);
        with_vsobject(&mut map, id, |v| assert!((v.point_ratio - 0.5).abs() < 1e-9));
    }
}
