import catalog from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json";

export const PLAY_HOST = catalog.host;
export const PLAY_HOST_VARIANT = catalog.hostVariant;
export const PLAY_GROUPS = catalog.groups;
export const PLAY_RUNTIME_PANES = catalog.groups.flatMap(group => group.panes.map(pane => ({ ...pane, group: group.id })));
