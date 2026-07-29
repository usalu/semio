/** @module Interface semio:framework/host **/
export function nowMs(): bigint;
export function backboneSend(uri: string, messageJson: string): void;
export function backbonePoll(uri: string): Array<string>;
