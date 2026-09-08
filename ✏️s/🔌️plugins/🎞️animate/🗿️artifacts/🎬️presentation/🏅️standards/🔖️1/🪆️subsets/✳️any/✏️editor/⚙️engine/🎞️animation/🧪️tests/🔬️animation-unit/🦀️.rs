mod tests {
    use super::*;
    use crate::editor::animate::engine::animation::animation::AnimateExt;
    use crate::editor::animate::engine::scene::sobject::VSobject;

    #[test]
    fn succession_lazy_activation_order() {
        let a1: Animations = Wait::new(1.0).into();
        let a2: Animations = Wait::new(1.0).into();
        let mut s = Succession::new(vec![a1, a2]);
        s.interpolate_mobject(0.25);
        s.interpolate_mobject(0.75);
        assert!(s.active_index.is_some());
    }

    #[test]
    fn animation_group_parallel_duration_is_max() {
        let g = AnimationGroup::new(vec![Wait::new(2.0).into(), Wait::new(5.0).into()]);
        assert!((g.duration() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn animate_builder_reads_target_id() {
        let mut v = VSobject::new();
        let id = v.id();
        let anim = v.animate(1.0).fade_in();
        assert_eq!(anim.target_id, id);
    }
}
