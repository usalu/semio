/** 🔬️ Minimal reproduction of the TS2367 family in the hub browser-broker oracle: a counter mutated only by a server callback stays narrowed to the literal asserted by the previous guard. */
type OracleUpstream = { effects: number };

function observedCount(counter: number): number {
  return counter;
}

export function probe(serve: (handler: () => void) => void): string {
  const state: OracleUpstream = { effects: 0 };
  serve(() => {
    state.effects += 1;
  });
  if (state.effects !== 0) throw new Error("armed before first request");
  if (state.effects !== 1) throw new Error("narrowed read");
  if (observedCount(state.effects) !== 1) throw new Error("function read");
  return "ok";
}
