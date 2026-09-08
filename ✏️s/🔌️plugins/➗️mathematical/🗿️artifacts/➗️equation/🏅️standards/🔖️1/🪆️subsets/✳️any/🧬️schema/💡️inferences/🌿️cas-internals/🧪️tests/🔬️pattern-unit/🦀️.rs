mod tests {
    use super::*;
    use crate::cas::expr::Expr;

    #[semio_framework_async_macros::async_test]
    async fn wildcard_any_matches_anything() {
        let pattern = wild(0);
        let subject = Expr::symbol("x");
        let bindings = match_expr(&pattern, &subject).unwrap();
        assert_eq!(bindings.get(&0), Some(&Binding::One(subject)));
    }

    #[semio_framework_async_macros::async_test]
    async fn wildcard_number_rejects_symbols() {
        let pattern = wild_num(0);
        assert!(match_expr(&pattern, &Expr::symbol("x")).is_none());
        assert!(match_expr(&pattern, &Expr::integer(5)).is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn structural_match_on_pow() {
        let x = Expr::symbol("x");
        let pattern = Expr::pow(wild(0), Expr::integer(2));
        let subject = Expr::pow(x.clone(), Expr::integer(2));
        let bindings = match_expr(&pattern, &subject).unwrap();
        assert_eq!(bindings.get(&0), Some(&Binding::One(x)));
    }

    #[semio_framework_async_macros::async_test]
    async fn pow_exponent_mismatch_fails() {
        let x = Expr::symbol("x");
        let pattern = Expr::pow(wild(0), Expr::integer(2));
        let subject = Expr::pow(x, Expr::integer(3));
        assert!(match_expr(&pattern, &subject).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn ac_match_finds_permuted_assignment() {
        // pattern: wild(0) + wild(1), subject: y + x -- should match regardless of order.
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let pattern = Expr::add(vec![wild(0), wild(1)]);
        let subject = Expr::add(vec![x.clone(), y.clone()]);
        let bindings = match_expr(&pattern, &subject).unwrap();
        let matched: std::collections::BTreeSet<Expr> = bindings
            .values()
            .map(|b| match b {
                Binding::One(e) => e.clone(),
                _ => panic!(),
            })
            .collect();
        assert!(matched.contains(&x) && matched.contains(&y));
    }

    #[semio_framework_async_macros::async_test]
    async fn seq_wildcard_absorbs_remaining_terms() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let z = Expr::symbol("z");
        let pattern = Expr::add(vec![x.clone(), wild_seq(0)]);
        let subject = Expr::add(vec![x, y.clone(), z.clone()]);
        let bindings = match_expr(&pattern, &subject).unwrap();
        match bindings.get(&0) {
            Some(Binding::Many(items)) => {
                let set: std::collections::BTreeSet<Expr> = items.iter().cloned().collect();
                assert!(set.contains(&y) && set.contains(&z));
            }
            _ => panic!("expected Many binding"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn rule_rewrites_matching_expression() {
        // sin(w)^2 + cos(w)^2 -> 1 (Pythagorean identity, single-term hand case without the +seq form)
        let w = Expr::symbol("w");
        let lhs = Expr::add(vec![Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Sin, vec![wild(0)]), Expr::integer(2)), Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Cos, vec![wild(0)]), Expr::integer(2))]);
        let rule = Rule::new(lhs, Expr::integer(1));
        let subject = Expr::add(vec![Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Sin, vec![w.clone()]), Expr::integer(2)), Expr::pow(Expr::func(crate::cas::fnkind::FnKind::Cos, vec![w]), Expr::integer(2))]);
        assert_eq!(rule.try_apply(&subject), Some(Expr::integer(1)));
    }

    #[semio_framework_async_macros::async_test]
    async fn ruleset_bottom_up_rewrites_nested_occurrence() {
        let rule = Rule::new(Expr::pow(wild(0), Expr::integer(2)), Expr::mul(vec![wild(0), wild(0)]));
        let rs = RuleSet::new(vec![rule]);
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x.clone(), Expr::integer(2)), Expr::integer(1)]);
        let result = rs.apply(&e, Strategy::BottomUpOnce);
        assert_eq!(result, Expr::add(vec![Expr::mul(vec![x.clone(), x]), Expr::integer(1)]));
    }

    #[semio_framework_async_macros::async_test]
    async fn free_of_constraint_rejects_expressions_containing_the_symbol() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let pattern = wild_free(0, "x");
        assert!(match_expr(&pattern, &y).is_some());
        assert!(match_expr(&pattern, &x).is_none());
    }
}
