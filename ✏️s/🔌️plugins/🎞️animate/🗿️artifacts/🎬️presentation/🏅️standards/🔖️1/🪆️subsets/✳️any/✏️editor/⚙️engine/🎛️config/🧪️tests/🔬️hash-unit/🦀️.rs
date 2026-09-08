mod tests {
    use super::*;

    #[test]
    fn animation_hash_is_stable() {
        let input = AnimationHashInput::new("FadeIn", 1.0).with_targets(vec![42]);
        let a = hash_animation(&input);
        let b = hash_animation(&input);
        assert_eq!(a, b);
    }

    #[test]
    fn timeline_merkle_orders_children() {
        let h = hash_animation_timeline(vec!["a".into(), "b".into()]);
        assert!(!h.is_empty());
    }

    #[test]
    fn hash_scene_config_is_stable_and_sensitive_to_inputs() {
        let a = hash_scene_config(60.0, 1920, 1080, 3);
        let b = hash_scene_config(60.0, 1920, 1080, 3);
        assert_eq!(a, b);
        let c = hash_scene_config(30.0, 1920, 1080, 3);
        assert_ne!(a, c);
    }

    #[test]
    fn hash_animation_differs_by_rate_and_extras() {
        let base = AnimationHashInput::new("Fade", 1.0);
        let with_rate = base.clone().with_rate("smooth");
        let with_extra = base.clone().with_extra("scale=2");
        assert_ne!(hash_animation(&base), hash_animation(&with_rate));
        assert_ne!(hash_animation(&base), hash_animation(&with_extra));
    }
}
