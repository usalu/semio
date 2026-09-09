use super::*;
use crate::wfc_engine::model::ModelBuilder;

fn model_and_domain(weights: &[f64]) -> (CompiledModel, Domain) {
    let mut b = ModelBuilder::new();
    for &w in weights {
        b.add_pattern(w);
    }
    b.add_relation("r");
    let model = b.compile().unwrap();
    let domain = Domain::new_full(model.weights());
    (model, domain)
}

#[test]
fn uniform_only_ever_returns_live_patterns() {
    let (model, domain) = model_and_domain(&[1.0, 1.0, 1.0]);
    let mut rng = Rng::from_seed(1);
    for _ in 0..50 {
        let p = sample_pattern(ValueSampler::Uniform, &domain, &model, &mut rng);
        assert!(domain.bits().get(p));
    }
}

#[test]
fn weighted_roulette_only_ever_returns_live_patterns() {
    let (model, domain) = model_and_domain(&[1.0, 5.0, 10.0]);
    let mut rng = Rng::from_seed(2);
    for _ in 0..50 {
        let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
        assert!(domain.bits().get(p));
    }
}

#[test]
fn weighted_roulette_is_biased_toward_heavier_pattern() {
    let (model, domain) = model_and_domain(&[1.0, 99.0]);
    let mut rng = Rng::from_seed(3);
    let mut counts = [0u32; 2];
    for _ in 0..2000 {
        let p = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut rng);
        counts[p.index()] += 1;
    }
    assert!(counts[1] > counts[0] * 10);
}

#[test]
fn same_seed_produces_same_sequence() {
    let (model, domain) = model_and_domain(&[1.0, 2.0, 3.0]);
    let mut r1 = Rng::from_seed(42);
    let mut r2 = Rng::from_seed(42);
    for _ in 0..20 {
        let p1 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r1);
        let p2 = sample_pattern(ValueSampler::WeightedRoulette, &domain, &model, &mut r2);
        assert_eq!(p1, p2);
    }
}
