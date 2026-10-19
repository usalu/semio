const Pass = ({ children }) => children ?? null;
export const SortableContext = Pass;
export const arrayMove = (list, from, to) => { const next = [...list]; next.splice(to, 0, next.splice(from, 1)[0]); return next; };
export const useSortable = () => ({ attributes: {}, listeners: {}, setNodeRef: () => undefined, transform: null, transition: undefined, isDragging: false });
export const verticalListSortingStrategy = () => null;
