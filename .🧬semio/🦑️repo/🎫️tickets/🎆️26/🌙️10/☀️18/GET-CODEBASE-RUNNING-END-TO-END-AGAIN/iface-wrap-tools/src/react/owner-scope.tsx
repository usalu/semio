/** 🔗️ The media-transport owner scope: which media owner (instance, controller, window and port) a media host below reads. The React lane binds it with a context that never leaves this module.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | MediaTransportHost}. The owner value is the lane-neutral `MediaTransportOwnerV1` of the media lifecycle.
 */

import type { MediaTransportOwnerV1 } from "@semio-tech/framework-os/modules/renderer/engine/media/lifecycle";
import { createContext, useContext, type ReactElement, type ReactNode } from "__PORT__";

const MediaTransportOwnerContext = createContext<MediaTransportOwnerV1 | null>(null);

/** 🪟️ Makes `owner` the media owner of everything rendered inside; `null` leaves the hosts below without one. */
export function MediaTransportOwnerScope({ owner, children }: { readonly owner: MediaTransportOwnerV1 | null; readonly children?: ReactNode }): ReactElement {
  return <MediaTransportOwnerContext.Provider value={owner}>{children}</MediaTransportOwnerContext.Provider>;
}

/** 🔎️ The media owner of the nearest scope, or `null` outside any. */
export function useMediaTransportOwner(): MediaTransportOwnerV1 | null {
  return useContext(MediaTransportOwnerContext);
}
