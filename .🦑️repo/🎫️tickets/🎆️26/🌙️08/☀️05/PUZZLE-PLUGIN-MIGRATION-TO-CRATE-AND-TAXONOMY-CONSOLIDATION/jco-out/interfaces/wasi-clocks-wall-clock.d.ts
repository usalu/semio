/** @module Interface wasi:clocks/wall-clock@0.2.9 **/
export function now(): Datetime;
export interface Datetime {
  seconds: bigint,
  nanoseconds: number,
}
