// #region Header

// Tree.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

// #region TODOs

// #endregion TODOs

import { closestCenter, DndContext, DragEndEvent } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@semio/js/components/ui/Collapsible";
import { ChevronDown, ChevronRight, GripVertical } from "lucide-react";
import { createContext, FC, ReactNode, useContext, useState } from "react";

const TreeContext = createContext<{ level: number }>({ level: 0 });

export interface TreeSectionAction {
  icon: ReactNode;
  onClick: () => void;
  title?: string;
}

interface TreeSectionProps {
  label: string;
  icon?: ReactNode;
  children?: ReactNode;
  defaultOpen?: boolean;
  className?: string;
  actions?: TreeSectionAction[];
}

interface SortableTreeItemProps {
  id: string;
  label?: ReactNode;
  icon?: ReactNode;
  children?: ReactNode;
  onClick?: () => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
}

interface TreeItemProps {
  label?: ReactNode;
  icon?: ReactNode;
  children?: ReactNode;
  onClick?: () => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  sortable?: boolean;
  sortableId?: string;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
}

interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => ReactNode;
}

export const TreeSection: FC<TreeSectionProps> = ({ label, icon, children, defaultOpen = true, className = "", actions = [] }) => {
  const [open, setOpen] = useState(defaultOpen);
  const [isHovered, setIsHovered] = useState(false);
  const hasChildren = Boolean(children);

  if (!hasChildren) {
    return (
      <div className={`flex items-center gap-1 py-1 px-2 hover:bg-muted select-none overflow-hidden group min-w-0 ${className}`} onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
        <div className="w-[14px] flex-shrink-0" />
        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
        <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{label}</span>
        {actions.length > 0 && (
          <div className="flex items-center gap-1">
            {actions.map((action, index) => (
              <button
                key={index}
                className={`p-1 rounded-sm transition-opacity hover:bg-muted-foreground/10 ${isHovered ? "opacity-100" : "opacity-30 group-hover:opacity-60"}`}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  action.onClick();
                }}
                title={action.title}
              >
                {action.icon}
              </button>
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <CollapsibleTrigger asChild>
        <div className={`flex items-center gap-1 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden group min-w-0 ${className}`} onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
          {open ? <ChevronDown size={14} className="flex-shrink-0" /> : <ChevronRight size={14} className="flex-shrink-0" />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{label}</span>
          {actions.length > 0 && (
            <div className="flex items-center gap-1">
              {actions.map((action, index) => (
                <button
                  key={index}
                  className={`p-1 rounded-sm transition-opacity hover:bg-muted-foreground/10 ${isHovered ? "opacity-100" : "opacity-30 group-hover:opacity-60"}`}
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  title={action.title}
                >
                  {action.icon}
                </button>
              ))}
            </div>
          )}
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent className="pl-2 min-w-0 overflow-hidden">{children}</CollapsibleContent>
    </Collapsible>
  );
};

const SortableTreeItem: FC<SortableTreeItemProps> = ({ id, label, icon, children, onClick, className = "", isSelected = false, isHighlighted = false, isDragHandle = false, defaultOpen = true }) => {
  const { level } = useContext(TreeContext);
  const [open, setOpen] = useState(defaultOpen);
  const hasChildren = Boolean(children);
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    paddingLeft: `${level * 1.25}rem`,
    opacity: isDragging ? 0.5 : 1,
  };

  const baseClasses = "flex items-center gap-1 py-0.5 px-1 hover:bg-muted cursor-pointer select-none overflow-hidden min-w-0";
  const stateClasses = `${isSelected ? "bg-accent" : ""} ${isHighlighted ? "bg-accent/50" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren) {
    return (
      <TreeContext.Provider value={{ level: level + 1 }}>
        {label && (
          <div
            ref={setNodeRef}
            style={style}
            className={itemClasses}
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              setOpen(!open);
              onClick?.();
            }}
          >
            {open ? <ChevronDown size={12} className="flex-shrink-0" /> : <ChevronRight size={12} className="flex-shrink-0" />}
            {isDragHandle && (
              <button className="cursor-grab active:cursor-grabbing p-0.5 hover:bg-muted-foreground/10 rounded" {...attributes} {...listeners} onClick={(e) => e.stopPropagation()}>
                <GripVertical size={12} className="text-muted-foreground" />
              </button>
            )}
            {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-xs font-normal truncate">{label}</span>
          </div>
        )}
        {open && <div className="pb-0.5 min-w-0 overflow-hidden">{children}</div>}
      </TreeContext.Provider>
    );
  }

  if (!label) {
    return <div className="min-w-0 overflow-hidden">{children}</div>;
  }

  return (
    <div ref={setNodeRef} style={style} className={itemClasses} onClick={onClick}>
      {isDragHandle && (
        <button className="cursor-grab active:cursor-grabbing p-0.5 hover:bg-muted-foreground/10 rounded" {...attributes} {...listeners}>
          <GripVertical size={12} className="text-muted-foreground" />
        </button>
      )}
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate">{label}</span>
    </div>
  );
};

export const SortableTreeItems: FC<SortableTreeItemsProps> = ({ items, onReorder, children }) => {
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (over && active.id !== over.id) {
      const oldIndex = items.findIndex((item) => item.id === active.id);
      const newIndex = items.findIndex((item) => item.id === over.id);
      if (oldIndex !== -1 && newIndex !== -1) {
        onReorder(oldIndex, newIndex);
      }
    }
  };

  return (
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map((item) => item.id)} strategy={verticalListSortingStrategy}>
        {items.map((item, index) => children(item, index))}
      </SortableContext>
    </DndContext>
  );
};

export const TreeItem: FC<TreeItemProps> = ({ label, icon, children, onClick, className = "", isSelected = false, isHighlighted = false, sortable = false, sortableId, isDragHandle = false, defaultOpen = true }) => {
  if (sortable && sortableId) {
    return <SortableTreeItem id={sortableId} label={label} icon={icon} children={children} onClick={onClick} className={className} isSelected={isSelected} isHighlighted={isHighlighted} isDragHandle={isDragHandle} defaultOpen={defaultOpen} />;
  }

  const { level } = useContext(TreeContext);
  const [open, setOpen] = useState(defaultOpen);
  const hasChildren = Boolean(children);
  const indentStyle = { paddingLeft: `${level * 1.25}rem` };
  const baseClasses = "flex items-center gap-1 py-0.5 px-1 hover:bg-muted cursor-pointer select-none overflow-hidden min-w-0";
  const stateClasses = `${isSelected ? "bg-accent" : ""} ${isHighlighted ? "bg-accent/50" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren) {
    return (
      <TreeContext.Provider value={{ level: level + 1 }}>
        {label && (
          <div
            className={itemClasses}
            style={indentStyle}
            onClick={(e) => {
              e.preventDefault();
              e.stopPropagation();
              setOpen(!open);
              onClick?.();
            }}
          >
            {open ? <ChevronDown size={12} className="flex-shrink-0" /> : <ChevronRight size={12} className="flex-shrink-0" />}
            {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-xs font-normal truncate">{label}</span>
          </div>
        )}
        {open && <div className="pb-0.5 min-w-0 overflow-hidden">{children}</div>}
      </TreeContext.Provider>
    );
  }

  if (!label) {
    return <div className="min-w-0 overflow-hidden">{children}</div>;
  }

  return (
    <div className={itemClasses} style={indentStyle} onClick={onClick}>
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate">{label}</span>
    </div>
  );
};

export const Tree: FC<{ children: ReactNode; className?: string }> = ({ children, className = "" }) => {
  return (
    <TreeContext.Provider value={{ level: 0 }}>
      <div className={`w-full min-w-0 overflow-hidden ${className}`}>{children}</div>
    </TreeContext.Provider>
  );
};
