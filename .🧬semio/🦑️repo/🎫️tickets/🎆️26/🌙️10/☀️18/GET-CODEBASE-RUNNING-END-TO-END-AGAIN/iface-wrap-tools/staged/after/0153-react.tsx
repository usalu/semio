/** 🧰️ The shared hub connection helpers: the live ref hook the connection callbacks read. */
import { useRef } from "../../⚛️react-port/🟦️.ts";

export function useLiveRef<T>(value: T): { current: T } {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}
