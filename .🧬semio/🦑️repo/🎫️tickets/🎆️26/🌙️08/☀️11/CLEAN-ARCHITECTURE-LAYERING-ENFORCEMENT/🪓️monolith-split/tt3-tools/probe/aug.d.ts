export {};
declare module "bun:test" {
  interface Matchers<T> {
    toBe(expected: unknown): void;
    toEqual(expected: unknown): void;
    toStrictEqual(expected: unknown): void;
    toContain(expected: unknown): void;
    toContainEqual(expected: unknown): void;
  }
}
