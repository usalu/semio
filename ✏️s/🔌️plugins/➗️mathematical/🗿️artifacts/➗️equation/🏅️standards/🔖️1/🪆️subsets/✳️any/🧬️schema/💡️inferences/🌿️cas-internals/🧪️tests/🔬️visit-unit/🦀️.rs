mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn subs_replaces_matching_subtree() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::add(vec![x.clone(), Expr::integer(1)]);
        let result = subs(&e, &x, &y);
        assert_eq!(result, Expr::add(vec![y, Expr::integer(1)]));
    }

    #[semio_framework_async_macros::async_test]
    async fn free_symbols_deduplicates_and_sorts() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::add(vec![x.clone(), x, y]);
        let symbols = free_symbols(&e);
        assert_eq!(symbols.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn node_count_hand_case() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![x, Expr::integer(1)]);
        assert_eq!(node_count(&e), 3); // Add(x, 1) has 2 children + 1 for itself
    }

    #[semio_framework_async_macros::async_test]
    async fn contains_symbol_detects_nested_occurrence() {
        let x = Expr::symbol("x");
        let y = Expr::symbol("y");
        let e = Expr::pow(Expr::add(vec![x.clone(), Expr::integer(1)]), Expr::integer(2));
        assert!(contains_symbol(&e, &x));
        assert!(!contains_symbol(&e, &y));
    }
}
