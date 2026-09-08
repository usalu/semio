mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn display_simple_polynomial() {
        let x = Expr::symbol("x");
        let e = Expr::add(vec![Expr::pow(x, Expr::integer(2)), Expr::integer(1)]);
        assert_eq!(display_string(&e), "x^2 + 1");
    }

    #[semio_framework_async_macros::async_test]
    async fn display_negative_term() {
        let x = Expr::symbol("x");
        let e = x - Expr::integer(1);
        assert_eq!(display_string(&e), "x - 1");
    }

    #[semio_framework_async_macros::async_test]
    async fn display_division() {
        let x = Expr::symbol("x");
        let e = x / Expr::integer(2);
        assert_eq!(display_string(&e), "x/2");
    }

    #[semio_framework_async_macros::async_test]
    async fn latex_fraction_and_power() {
        let x = Expr::symbol("x");
        let e = Expr::pow(x, Expr::integer(2));
        assert_eq!(to_latex(&e), "{x}^{2}");
    }

    #[semio_framework_async_macros::async_test]
    async fn latex_constant_pi() {
        assert_eq!(to_latex(&Expr::constant(Constant::Pi)), "\\pi");
    }
}
