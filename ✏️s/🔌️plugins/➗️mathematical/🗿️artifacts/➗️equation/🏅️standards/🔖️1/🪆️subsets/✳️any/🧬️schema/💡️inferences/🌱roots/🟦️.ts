/** 🌱 `roots` — the equation's real roots, isolated (Sturm sequences) and bisection-refined to
 * 1/10^9. Scope: single-variable, integer-coefficient equations only (see the Rust component's
 * doc header) — anything else infers an empty list, never a wrong or missing value. */

export interface EquationRoot {
  approx: number;
}
