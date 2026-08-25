/** @module Interface semio:framework/capabilities@1.0.0 **/
export type CapabilityId = string;
export interface CapabilityToken {
  id: CapabilityId,
  token: bigint,
}
export interface CapabilityGrant {
  token: CapabilityToken,
  scope: string,
  expiresMs?: bigint,
}
export type CapabilityChange = CapabilityChangeGranted | CapabilityChangeRevoked | CapabilityChangeNarrowed;
export interface CapabilityChangeGranted {
  tag: 'granted',
  val: CapabilityGrant,
}
export interface CapabilityChangeRevoked {
  tag: 'revoked',
  val: CapabilityId,
}
export interface CapabilityChangeNarrowed {
  tag: 'narrowed',
  val: CapabilityGrant,
}
