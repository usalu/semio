const Pass = ({ children }) => children ?? null;
const noop = () => undefined;
export const DndContext = Pass, DragOverlay = Pass, PointerSensor = class {};
export const closestCenter = noop, pointerWithin = noop, rectIntersection = noop;
export const useDraggable = () => ({ attributes: {}, listeners: {}, setNodeRef: noop, transform: null, isDragging: false });
export const useDroppable = () => ({ setNodeRef: noop, isOver: false });
export const useSensor = noop, useSensors = () => [];
