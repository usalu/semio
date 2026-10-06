/** 🎯️ Qualifies the exact owned RGBA8 target size without allocating GPU resources. */
export function canvasRasterTargetSpec(width: number, height: number): { bytes: string | null; refusal: string | null } {
  if (!Number.isInteger(width) || !Number.isInteger(height) || width < 0 || height < 0 || width > 4294967295 || height > 4294967295) throw new RangeError("invalid target dimension");
  if (width === 0 || height === 0) return { bytes: null, refusal: "empty-target" };
  const pixels = BigInt(width) * BigInt(height);
  if (pixels > 4611686018427387903n) return { bytes: null, refusal: "target-byte-overflow" };
  return { bytes: (pixels * 4n).toString(), refusal: null };
}
