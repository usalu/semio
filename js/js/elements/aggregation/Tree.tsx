// #region Header

// Tree.tsx
//
// Hierarchical tree component with multiple variants:
//
// - Tree (default): Flexible tree with TreeSection and TreeItem children
// - Tree.Files: File/folder tree with automatic icons and navigation
//
// Architecture:
// - Tree.tsx: Generic UI components (no app dependencies)
// - App-specific wrappers: Use Tree.Files with app logic (routing, data fetching)
//
// Example app-specific wrapper:
// ```tsx
// const SectionTree = () => {
//   const tree = docsRegistry.getSectionTree(section);
//   return <Tree.Files nodes={tree} onNavigate={navigate} />;
// };
// ```

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

// TODO: Remove TreeContent boilerplate.

// #endregion TODOs

import { closestCenter, DndContext, DragEndEvent } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { ChevronDown, ChevronRight, FileText, Folder, GripVertical } from "lucide-react";
import { Children, createContext, FC, isValidElement, ReactNode, useContext, useState, type MouseEvent as ReactMouseEvent } from "react";
import { useTranslation } from "react-i18next";
import { IdTooltipContent, Tooltip, TooltipContent, TooltipTrigger, useTooltipMode } from "../display/Tooltip";
import { Action } from "../input/Action";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "./Collapsible";
import { TreeStateProvider, useTreeState } from "./TreeStateProvider";

const hasNonEmptyChildren = (children: ReactNode): boolean => {
  if (!children) return false;
  const childArray = Children.toArray(children);
  return (
    childArray.length > 0 &&
    childArray.some((child) => {
      if (isValidElement(child)) return true;
      if (typeof child === "string" && child.trim().length > 0) return true;
      if (typeof child === "number") return true;
      return false;
    })
  );
};

const TreeContext = createContext<{ level: number; isLastAtLevel: boolean[]; showLines: boolean }>({ level: 0, isLastAtLevel: [], showLines: true });

const IndentationLines: FC<{ level: number; isLastAtLevel: boolean[]; showLines: boolean }> = ({ level, isLastAtLevel, showLines }) => {
  if (!showLines || level === 0) return null;

  return (
    <div className="absolute left-0 top-0 bottom-0 pointer-events-none">
      {Array.from({ length: level }, (_, i) => (
        <div key={i} className="absolute top-0 bottom-0" style={{ left: `${i * 0.75 + 0.375}rem` }}>
          {!isLastAtLevel[i] && <div className="w-px h-full bg-muted-foreground/40" />}
        </div>
      ))}
    </div>
  );
};

// Wrapper component that applies indentation to custom content (non-TreeItem children)
export const TreeContent: FC<{ children: ReactNode }> = ({ children }) => {
  const { level, isLastAtLevel, showLines } = useContext(TreeContext);
  return (
    <div className="relative py-0.5" style={{ paddingLeft: `${level * 0.75}rem` }}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {children}
    </div>
  );
};

export interface TreeSectionAction {
  icon: ReactNode;
  onClick: () => void;
  title?: string;
  id?: string;
}

interface TreeSectionProps {
  label?: string;
  id?: string;
  icon?: ReactNode;
  children?: ReactNode;
  defaultOpen?: boolean;
  className?: string;
  actions?: TreeSectionAction[];
  onPointerEnter?: () => void;
  onPointerLeave?: () => void;
  onDoubleClick?: (event: ReactMouseEvent) => void;
}

interface SortableTreeItemProps {
  id: string;
  label?: ReactNode;
  icon?: ReactNode;
  children?: ReactNode;
  onClick?: (event: ReactMouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  isLastItem?: boolean;
  actions?: TreeSectionAction[];
  onDoubleClick?: (event: ReactMouseEvent) => void;
}

interface TreeItemProps {
  label?: ReactNode;
  id?: string;
  icon?: ReactNode;
  children?: ReactNode;
  onClick?: (event: ReactMouseEvent) => void;
  className?: string;
  isSelected?: boolean;
  isHighlighted?: boolean;
  sortable?: boolean;
  sortableId?: string;
  isDragHandle?: boolean;
  defaultOpen?: boolean;
  isLastItem?: boolean;
  actions?: TreeSectionAction[];
  onDoubleClick?: (event: ReactMouseEvent) => void;
}

interface SortableTreeItemsProps {
  items: { id: string; [key: string]: any }[];
  onReorder: (oldIndex: number, newIndex: number) => void;
  children: (item: any, index: number) => ReactNode;
}

export const TreeSection: FC<TreeSectionProps> = ({ label, id, icon, children, defaultOpen = true, className = "", actions = [], onPointerEnter: onSectionPointerEnter, onPointerLeave: onSectionPointerLeave, onDoubleClick }) => {
  const { level, isLastAtLevel, showLines } = useContext(TreeContext);
  const treeState = useTreeState();
  const { t } = useTranslation();
  const mode = useTooltipMode();
  const displayLabel = id ? t(`${id}.label`) : label;
  const sectionId = `section-${displayLabel}`;
  const open = treeState.getOpenState(sectionId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(sectionId, value);
  const [isHovered, setIsHovered] = useState(false);
  const hasChildren = hasNonEmptyChildren(children);

  if (!hasChildren) {
    return (
      <div
        className={`relative flex items-center gap-1 py-1 hover:bg-hover-panel select-none overflow-hidden group min-w-0 cursor-selectable ${className}`}
        style={{ paddingLeft: `${level * 0.75}rem` }}
        onPointerEnter={() => {
          setIsHovered(true);
          onSectionPointerEnter?.();
        }}
        onPointerLeave={() => {
          setIsHovered(false);
          onSectionPointerLeave?.();
        }}
        onDoubleClick={(event) => {
          if (!onDoubleClick) return;
          event.preventDefault();
          event.stopPropagation();
          onDoubleClick(event);
        }}
      >
        <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
        <div className="w-[14px] flex-shrink-0" />
        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
        {id ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
            </TooltipTrigger>
            <TooltipContent>
              <IdTooltipContent id={id} mode={mode} />
            </TooltipContent>
          </Tooltip>
        ) : (
          <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
        )}
        {actions.length > 0 && (
          <div className="flex items-center gap-0.5">
            {actions.map((action, index) => (
              <Action
                key={index}
                level="panel"
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  action.onClick();
                }}
                id={action.id}
              >
                {action.icon}
              </Action>
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <CollapsibleTrigger asChild>
        <div
          className={`relative flex items-center gap-1 py-1 hover:bg-hover-panel select-none overflow-hidden group min-w-0 cursor-foldable ${className}`}
          style={{ paddingLeft: `${level * 0.75}rem` }}
          role="button"
          onPointerEnter={() => {
            setIsHovered(true);
            onSectionPointerEnter?.();
          }}
          onPointerLeave={() => {
            setIsHovered(false);
            onSectionPointerLeave?.();
          }}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          {open ? <ChevronDown size={14} className="flex-shrink-0" /> : <ChevronRight size={14} className="flex-shrink-0" />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          {id ? (
            <Tooltip>
              <TooltipTrigger asChild>
                <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
              </TooltipTrigger>
              <TooltipContent>
                <IdTooltipContent id={id} mode={mode} />
              </TooltipContent>
            </Tooltip>
          ) : (
            <span className="flex-1 text-xs text-muted-foreground uppercase tracking-wide truncate">{displayLabel}</span>
          )}
          {actions.length > 0 && (
            <div className="flex items-center gap-0.5">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  level="panel"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                >
                  {action.icon}
                </Action>
              ))}
            </div>
          )}
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent className="min-w-0 overflow-hidden">
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines }}>{children}</TreeContext.Provider>
      </CollapsibleContent>
    </Collapsible>
  );
};

const SortableTreeItem: FC<SortableTreeItemProps> = ({ id, label, icon, children, onClick, className = "", isSelected = false, isHighlighted = false, isDragHandle = false, defaultOpen = true, isLastItem = false, actions = [], onDoubleClick }) => {
  const { level, isLastAtLevel, showLines } = useContext(TreeContext);
  const treeState = useTreeState();
  const { t } = useTranslation();
  const displayLabel = id ? t(`${id}.label`, { defaultValue: t(id) }) : label;
  const itemKey = id ?? displayLabel ?? id;
  const itemId = `item-${id}-${itemKey}`;
  const open = treeState.getOpenState(itemId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);
  const [isHovered, setIsHovered] = useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
    paddingLeft: `${level * 0.75}rem`,
  };

  const baseClasses = `relative flex items-center gap-1 py-0.5 hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-accent" : ""} ${isHighlighted ? "bg-accent/50" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && displayLabel) {
    return (
      <>
        <div
          ref={setNodeRef}
          style={style}
          className={itemClasses}
          onClick={(e) => {
            if (e.detail > 1) return;
            e.preventDefault();
            e.stopPropagation();
            setOpen(!open);
            onClick?.(e);
          }}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
          onMouseEnter={() => setIsHovered(true)}
          onMouseLeave={() => setIsHovered(false)}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          {open ? <ChevronDown size={12} className="flex-shrink-0" /> : <ChevronRight size={12} className="flex-shrink-0" />}
          {isDragHandle && (
            <Action level="panel" className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners} onClick={(e) => e.stopPropagation()}>
              <GripVertical size={12} className="text-muted-foreground" />
            </Action>
          )}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span className="flex-1 text-xs font-normal truncate text-foreground">{displayLabel as ReactNode}</span>
          {actions.length > 0 && (
            <div className="flex items-center gap-0.5">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  level="panel"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                >
                  {action.icon}
                </Action>
              ))}
            </div>
          )}
        </div>
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines }}>{children}</TreeContext.Provider>}
      </>
    );
  }

  if (!displayLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines }}>{children}</TreeContext.Provider>;
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={itemClasses}
      onClick={(event) => {
        if (event.detail > 1) return;
        onClick?.(event);
      }}
      onDoubleClick={(event) => {
        if (!onDoubleClick) return;
        event.preventDefault();
        event.stopPropagation();
        onDoubleClick(event);
      }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {isDragHandle && (
        <Action level="panel" className="cursor-grab active:cursor-grabbing" {...attributes} {...listeners}>
          <GripVertical size={12} className="text-muted-foreground" />
        </Action>
      )}
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate text-foreground">{displayLabel as ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-0.5">
          {actions.map((action, index) => (
            <Action
              key={index}
              level="panel"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                action.onClick();
              }}
              id={action.id}
            >
              {action.icon}
            </Action>
          ))}
        </div>
      )}
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

export const TreeItem: FC<TreeItemProps> = ({
  label,
  id,
  icon,
  children,
  onClick,
  className = "",
  isSelected = false,
  isHighlighted = false,
  sortable = false,
  sortableId,
  isDragHandle = false,
  defaultOpen = true,
  isLastItem = false,
  actions = [],
  onDoubleClick,
}) => {
  const { t } = useTranslation();
  const resolvedLabel = id ? t(`${id}.label`, { defaultValue: t(id) }) : label;
  if (sortable && sortableId) {
    return (
      <SortableTreeItem
        id={sortableId}
        label={resolvedLabel}
        icon={icon}
        children={children}
        onClick={onClick}
        className={className}
        isSelected={isSelected}
        isHighlighted={isHighlighted}
        isDragHandle={isDragHandle}
        defaultOpen={defaultOpen}
        isLastItem={isLastItem}
        actions={actions}
        onDoubleClick={onDoubleClick}
      />
    );
  }

  const { level, isLastAtLevel, showLines } = useContext(TreeContext);
  const treeState = useTreeState();
  const itemKey = id ?? resolvedLabel ?? sortableId ?? "tree-item";
  const itemId = `item-${itemKey}`;
  const open = treeState.getOpenState(itemId, defaultOpen);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);
  const [isHovered, setIsHovered] = useState(false);
  const hasChildren = hasNonEmptyChildren(children);
  const baseClasses = `relative flex items-center gap-1 py-0.5 hover:bg-hover-panel select-none overflow-hidden min-w-0 group ${hasChildren ? "cursor-foldable" : "cursor-selectable"}`;
  const stateClasses = `${isSelected ? "bg-accent" : ""} ${isHighlighted ? "bg-accent/50" : ""}`;
  const itemClasses = `${baseClasses} ${stateClasses} ${className}`;

  if (hasChildren && resolvedLabel) {
    return (
      <>
        <div
          className={itemClasses}
          style={{ paddingLeft: `${level * 0.75}rem` }}
          onClick={(e) => {
            if (e.detail > 1) return;
            e.preventDefault();
            e.stopPropagation();
            setOpen(!open);
            onClick?.(e);
          }}
          onDoubleClick={(event) => {
            if (!onDoubleClick) return;
            event.preventDefault();
            event.stopPropagation();
            onDoubleClick(event);
          }}
          onMouseEnter={() => setIsHovered(true)}
          onMouseLeave={() => setIsHovered(false)}
        >
          <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
          {open ? <ChevronDown size={12} className="flex-shrink-0" /> : <ChevronRight size={12} className="flex-shrink-0" />}
          {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span className="flex-1 text-xs font-normal truncate text-foreground">{resolvedLabel as ReactNode}</span>
          {actions.length > 0 && (
            <div className="flex items-center gap-0.5">
              {actions.map((action, index) => (
                <Action
                  key={index}
                  level="panel"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    action.onClick();
                  }}
                  id={action.id}
                >
                  {action.icon}
                </Action>
              ))}
            </div>
          )}
        </div>
        {open && <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, isLastItem], showLines }}>{children}</TreeContext.Provider>}
      </>
    );
  }

  if (!resolvedLabel) {
    return <TreeContext.Provider value={{ level, isLastAtLevel, showLines }}>{children}</TreeContext.Provider>;
  }

  return (
    <div className={itemClasses} style={{ paddingLeft: `${level * 0.75}rem` }} onClick={onClick} onMouseEnter={() => setIsHovered(true)} onMouseLeave={() => setIsHovered(false)}>
      <IndentationLines level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} />
      {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-xs font-normal truncate text-foreground">{resolvedLabel as ReactNode}</span>
      {actions.length > 0 && (
        <div className="flex items-center gap-0.5">
          {actions.map((action, index) => (
            <Action
              key={index}
              level="panel"
              onClick={(e) => {
                e.preventDefault();
                e.stopPropagation();
                action.onClick();
              }}
              id={action.id}
            >
              {action.icon}
            </Action>
          ))}
        </div>
      )}
    </div>
  );
};

export const TreeItems: FC<{ children: ReactNode[]; renderItem: (child: ReactNode, index: number, isLast: boolean) => ReactNode }> = ({ children, renderItem }) => {
  return <>{children.map((child, index) => renderItem(child, index, index === children.length - 1))}</>;
};

// File tree data structure for displaying files and folders
export interface FileTreeNode {
  title: string;
  path: string;
  isFolder: boolean;
  children?: FileTreeNode[];
}

export const Tree: FC<{ children: ReactNode; className?: string; showLines?: boolean }> = ({ children, className = "", showLines = true }) => {
  return (
    <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines }}>
      <div className={`w-full min-w-0 overflow-hidden ${className}`}>{children}</div>
    </TreeContext.Provider>
  );
};

interface FileTreeItemProps {
  node: FileTreeNode;
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
}

const FileTreeItem: FC<FileTreeItemProps> = ({ node, currentPath, onNavigate, as = "a" }) => {
  const { level } = useContext(TreeContext);
  const [isHovered, setIsHovered] = useState(false);
  const treeState = useTreeState();
  const itemId = `file-${node.path}`;
  const open = treeState.getOpenState(itemId, true);
  const setOpen = (value: boolean) => treeState.setOpenState(itemId, value);

  const isActive = currentPath === node.path;
  const hasChildren = node.children && node.children.length > 0;
  const Icon = node.isFolder ? Folder : FileText;

  const baseClasses = "relative flex items-center gap-2 py-1.5 px-3 rounded-md hover:bg-accent transition-colors cursor-selectable select-none";
  const stateClasses = isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground hover:text-foreground";
  const itemClasses = `${baseClasses} ${stateClasses}`;

  const handleClick = (e: ReactMouseEvent) => {
    if (hasChildren) {
      e.preventDefault();
      setOpen(!open);
    }
    if (onNavigate) {
      onNavigate(node.path);
    }
  };

  const content = (
    <>
      <Icon className="w-4 h-4 shrink-0" />
      <span className="text-sm">{node.title}</span>
    </>
  );

  const sharedProps = {
    className: itemClasses,
    style: { paddingLeft: `${level * 1 + 0.75}rem` },
    onClick: handleClick,
    onMouseEnter: () => setIsHovered(true),
    onMouseLeave: () => setIsHovered(false),
  };

  const itemElement =
    as === "a" ? (
      <a href={`/${node.path}`} {...sharedProps}>
        {content}
      </a>
    ) : (
      <div {...sharedProps}>{content}</div>
    );

  if (hasChildren && node.isFolder) {
    return (
      <>
        {itemElement}
        {open && (
          <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [], showLines: false }}>
            {node.children!.map((child, idx) => (
              <FileTreeItem key={idx} node={child} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </TreeContext.Provider>
        )}
      </>
    );
  }

  return itemElement;
};

interface TreeFilesProps {
  title?: string;
  nodes: FileTreeNode[];
  currentPath?: string;
  onNavigate?: (path: string) => void;
  as?: "a" | "div";
  className?: string;
}

/**
 * Tree.Files - A variant of Tree for displaying file/folder structures
 * with automatic icons and navigation support.
 *
 * @example
 * <Tree.Files
 *   nodes={fileNodes}
 *   currentPath="docs/getting-started"
 *   onNavigate={(path) => navigate(path)}
 * />
 */
Tree.Files = ({ title = "In this section", nodes, currentPath, onNavigate, as = "a", className = "" }: TreeFilesProps) => {
  if (nodes.length === 0) return null;

  return (
    <TreeStateProvider>
      <div className={`not-prose my-8 p-6 rounded-lg border border-border bg-card ${className}`}>
        {title && <h3 className="text-lg font-semibold mb-4">{title}</h3>}
        <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: false }}>
          <div className="flex flex-col gap-0.5">
            {nodes.map((node, idx) => (
              <FileTreeItem key={idx} node={node} currentPath={currentPath} onNavigate={onNavigate} as={as} />
            ))}
          </div>
        </TreeContext.Provider>
      </div>
    </TreeStateProvider>
  );
};

/**
 * Tree.Section - Same as Tree.Files but with a semantic name for section navigation
 *
 * This is used by apps to show a section's file tree. The name makes it clear
 * that this is for section/folder navigation rather than generic file display.
 *
 * @example
 * <Tree.Section
 *   nodes={sectionNodes}
 *   currentPath="docs/getting-started"
 *   onNavigate={(path) => navigate(path)}
 * />
 */
Tree.Section = Tree.Files;

// Backward compatibility: Export FileTree as an alias for Tree.Files
export const FileTree = Tree.Files;
