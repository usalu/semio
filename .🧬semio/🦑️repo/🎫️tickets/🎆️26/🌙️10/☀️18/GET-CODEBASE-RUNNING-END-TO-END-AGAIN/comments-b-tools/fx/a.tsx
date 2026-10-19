// license header
import { a } from "x"; // trailing import

/** 📦 doc */
export function f(x: number) { // open
  // standalone
  const s = "// not a comment";
  const t = `${x} // not ${/* inner */ x}`; // trailing
  /* block */ const r = /\/\/ re/.test(s);

  // @ts-expect-error keep
  const u: number = "a";
  /* eslint-disable-next-line */
  return <div>
    {/* jsx comment */}
    // not a comment in jsx text
    <b>a {/* inline */} b</b>
  </div>; /* tail */
}

const X = {
  a: 1, // one
  // two
  b: 2,
};

const y = 5; // top-level trailing stays
// top-level stays
class C {
  /** doc */
  m() {
    // c
  }
  // between
  n() {}
}
