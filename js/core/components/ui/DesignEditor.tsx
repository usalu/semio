// #region Header

// DesignEditor.tsx

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

import { DndContext, DragEndEvent, DragOverlay, DragStartEvent, useDraggable } from '@dnd-kit/core'
import { ReactFlowProvider, useReactFlow } from '@xyflow/react'
import { ChevronDown, ChevronRight, Info, MessageCircle, Terminal, Wrench } from 'lucide-react'
import { FC, ReactNode, useCallback, useEffect, useMemo, useReducer, useState } from 'react'
import { createPortal } from 'react-dom'
import { useHotkeys } from 'react-hotkeys-hook'

import {
  Design, DesignDiff, DesignId, Diagram,
  ICON_WIDTH, Kit, Model, Piece, Type,
  addConnectionToDesign,
  addConnectionToDesignDiff,
  addConnectionToSelection,
  addConnectionsToDesign,
  addConnectionsToDesignDiff,
  addConnectionsToSelection,
  addPieceToDesign,
  addPieceToDesignDiff,
  addPieceToSelection,
  addPiecesToDesign,
  addPiecesToDesignDiff,
  addPiecesToSelection,
  findDesign,
  removeConnectionFromDesign,
  removeConnectionFromDesignDiff,
  removeConnectionFromSelection,
  removeConnectionsFromDesign,
  removeConnectionsFromDesignDiff,
  removeConnectionsFromSelection,
  removePieceFromDesign,
  removePieceFromDesignDiff,
  removePieceFromSelection,
  removePiecesFromDesign,
  removePiecesFromDesignDiff,
  removePiecesFromSelection,
  sameDesign,
  setConnectionInDesign,
  setConnectionInDesignDiff,
  setConnectionsInDesign,
  setConnectionsInDesignDiff,
  setPieceInDesign,
  setPieceInDesignDiff,
  setPiecesInDesign,
  setPiecesInDesignDiff
} from '@semio/js'
import { Avatar, AvatarFallback } from '@semio/js/components/ui/Avatar'
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible'
import { HoverCard, HoverCardContent, HoverCardTrigger } from '@semio/js/components/ui/HoverCard'
import { Input } from '@semio/js/components/ui/Input'
import { default as Navbar } from '@semio/js/components/ui/Navbar'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@semio/js/components/ui/Resizable'
import { ScrollArea } from '@semio/js/components/ui/ScrollArea'
import { Layout, Mode, Theme } from '@semio/js/components/ui/Sketchpad'
import { Slider } from '@semio/js/components/ui/Slider'
import { Textarea } from '@semio/js/components/ui/Textarea'
import { ToggleGroup, ToggleGroupItem } from '@semio/js/components/ui/ToggleGroup'
import { Generator } from '@semio/js/lib/utils'

//#region State

export interface DesignEditorSelection {
  selectedPieceIds: string[];
  selectedConnections: {
    connectingPieceId: string;
    connectedPieceId: string;
  }[];
}

export interface DesignEditorState {
  kit: Kit
  designId: DesignId
  fileUrls: Map<string, string>
  fullscreenPanel: FullscreenPanel
  selection: DesignEditorSelection
  designDiff: DesignDiff
}

export enum FullscreenPanel {
  None = 'none',
  Diagram = 'diagram',
  Model = 'model'
}

export enum DesignEditorAction {
  SetDesign = 'SET_DESIGN',
  AddPieceToDesign = 'ADD_PIECE_TO_DESIGN',
  SetPieceInDesign = 'SET_PIECE_IN_DESIGN',
  RemovePieceFromDesign = 'REMOVE_PIECE_FROM_DESIGN',
  AddPiecesToDesign = 'ADD_PIECES_TO_DESIGN',
  SetPiecesInDesign = 'SET_PIECES_IN_DESIGN',
  RemovePiecesFromDesign = 'REMOVE_PIECES_FROM_DESIGN',
  AddConnectionToDesign = 'ADD_CONNECTION_TO_DESIGN',
  SetConnectionInDesign = 'SET_CONNECTION_IN_DESIGN',
  RemoveConnectionFromDesign = 'REMOVE_CONNECTION_FROM_DESIGN',
  AddConnectionsToDesign = 'ADD_CONNECTIONS_TO_DESIGN',
  SetConnectionsInDesign = 'SET_CONNECTIONS_IN_DESIGN',
  RemoveConnectionsFromDesign = 'REMOVE_CONNECTIONS_FROM_DESIGN',
  SetDesignDiff = 'SET_DESIGN_DIFF',
  AddPieceToDesignDiff = 'ADD_PIECE_TO_DESIGN_DIFF',
  SetPieceInDesignDiff = 'SET_PIECE_IN_DESIGN_DIFF',
  RemovePieceFromDesignDiff = 'REMOVE_PIECE_FROM_DESIGN_DIFF',
  AddPiecesToDesignDiff = 'ADD_PIECES_TO_DESIGN_DIFF',
  SetPiecesInDesignDiff = 'SET_PIECES_IN_DESIGN_DIFF',
  RemovePiecesFromDesignDiff = 'REMOVE_PIECES_FROM_DESIGN_DIFF',
  AddConnectionToDesignDiff = 'ADD_CONNECTION_TO_DESIGN_DIFF',
  SetConnectionInDesignDiff = 'SET_CONNECTION_IN_DESIGN_DIFF',
  RemoveConnectionFromDesignDiff = 'REMOVE_CONNECTION_FROM_DESIGN_DIFF',
  AddConnectionsToDesignDiff = 'ADD_CONNECTIONS_TO_DESIGN_DIFF',
  SetConnectionsInDesignDiff = 'SET_CONNECTIONS_IN_DESIGN_DIFF',
  RemoveConnectionsFromDesignDiff = 'REMOVE_CONNECTIONS_FROM_DESIGN_DIFF',
  SetSelection = 'SET_SELECTION',
  AddPieceToSelection = 'ADD_PIECE_TO_SELECTION',
  RemovePieceFromSelection = 'REMOVE_PIECE_FROM_SELECTION',
  AddPiecesToSelection = 'ADD_PIECES_TO_SELECTION',
  RemovePiecesFromSelection = 'REMOVE_PIECES_FROM_SELECTION',
  AddConnectionToSelection = 'ADD_CONNECTION_TO_SELECTION',
  RemoveConnectionFromSelection = 'REMOVE_CONNECTION_FROM_SELECTION',
  AddConnectionsToSelection = 'ADD_CONNECTIONS_TO_SELECTION',
  RemoveConnectionsFromSelection = 'REMOVE_CONNECTIONS_FROM_SELECTION',
  SetFullscreen = 'SET_FULLSCREEN'
}

export type DesignEditorDispatcher = (action: { type: DesignEditorAction; payload: any }) => void

//#endregion

//#region Panels

interface PanelToggles {
  workbench: boolean
  console: boolean
  details: boolean
  chat: boolean
}

interface PanelProps {
  visible: boolean
}

interface ResizablePanelProps extends PanelProps {
  onWidthChange?: (width: number) => void
  width: number
}

interface TreeNodeProps {
  label: ReactNode
  icon?: ReactNode
  children?: ReactNode
  level?: number
  collapsible?: boolean
  defaultOpen?: boolean
  isLeaf?: boolean
}

interface TreeSectionProps {
  label: string
  icon?: ReactNode
  children?: ReactNode
  defaultOpen?: boolean
}

const TreeSection: FC<TreeSectionProps> = ({ label, icon, children, defaultOpen = true }) => {
  const [open, setOpen] = useState(defaultOpen)

  return (
    <Collapsible open={open} onOpenChange={setOpen}>
      <CollapsibleTrigger asChild>
        <div className="flex items-center gap-2 py-1.5 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden">
          {open ? (
            <ChevronDown size={14} className="flex-shrink-0" />
          ) : (
            <ChevronRight size={14} className="flex-shrink-0" />
          )}
          {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
          <span className="flex-1 text-sm text-muted-foreground uppercase tracking-wide truncate">{label}</span>
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent>{children}</CollapsibleContent>
    </Collapsible>
  )
}

const TreeNode: FC<TreeNodeProps> = ({
  label,
  icon,
  children,
  level = 0,
  collapsible = false,
  defaultOpen = true,
  isLeaf = false
}) => {
  const [open, setOpen] = useState(defaultOpen)
  const indentStyle = { paddingLeft: `${level * 1.25}rem` }

  const Trigger = collapsible ? CollapsibleTrigger : 'div'
  const Content = collapsible ? CollapsibleContent : 'div'

  const triggerContent = (
    <div
      className="flex items-center gap-2 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden"
      style={indentStyle}
    >
      {collapsible &&
        !isLeaf &&
        (open ? (
          <ChevronDown size={14} className="flex-shrink-0" />
        ) : (
          <ChevronRight size={14} className="flex-shrink-0" />
        ))}
      {icon && <span className="w-4 h-4 flex items-center justify-center flex-shrink-0">{icon}</span>}
      <span className="flex-1 text-sm font-normal truncate">{label}</span>
    </div>
  )

  if (collapsible) {
    return (
      <Collapsible open={open} onOpenChange={setOpen}>
        <Trigger asChild>{triggerContent}</Trigger>
        <Content>{children}</Content>
      </Collapsible>
    )
  } else if (isLeaf) {
    return triggerContent
  } else {
    return (
      <>
        {triggerContent}
        {children}
      </>
    )
  }
}

interface TypeAvatarProps {
  type: Type
  showHoverCard?: boolean
}

const TypeAvatar: FC<TypeAvatarProps> = ({ type, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `type-${type.name}-${type.variant || ''}`
  })

  const displayVariant = type.variant || type.name
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  )

  if (!showHoverCard) {
    return avatar
  }

  return (
    <HoverCard>
      <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          {type.variant ? (
            <>
              <h4 className="text-sm font-semibold">{type.variant}</h4>
              <p className="text-sm">{type.description || 'No description available.'}</p>
            </>
          ) : (
            <p className="text-sm">{type.description || 'No description available.'}</p>
          )}
        </div>
      </HoverCardContent>
    </HoverCard>
  )
}

interface DesignAvatarProps {
  design: Design
  showHoverCard?: boolean
}

const DesignAvatar: FC<DesignAvatarProps> = ({ design, showHoverCard = false }) => {
  const { attributes, listeners, setNodeRef } = useDraggable({
    id: `design-${design.name}-${design.variant || ''}-${design.view || ''}`
  })

  // Determine if this is the default variant and view
  const isDefault = (!design.variant || design.variant === design.name) && (!design.view || design.view === 'Default')

  const displayVariant = design.variant || design.name
  const avatar = (
    <Avatar ref={setNodeRef} {...listeners} {...attributes} className="cursor-grab">
      {/* <AvatarImage src="https://github.com/semio-tech.png" /> */}
      <AvatarFallback>{displayVariant.substring(0, 2).toUpperCase()}</AvatarFallback>
    </Avatar>
  )

  if (!showHoverCard) {
    return avatar
  }

  return (
    <HoverCard>
      <HoverCardTrigger asChild>{avatar}</HoverCardTrigger>
      <HoverCardContent className="w-80">
        <div className="space-y-1">
          {!isDefault && (
            <h4 className="text-sm font-semibold">
              {design.variant || design.name}
              {design.view && design.view !== 'Default' && ` (${design.view})`}
            </h4>
          )}
          <p className="text-sm">{design.description || 'No description available.'}</p>
        </div>
      </HoverCardContent>
    </HoverCard>
  )
}

interface WorkbenchProps extends ResizablePanelProps {
  kit: Kit
}

const Workbench: FC<WorkbenchProps> = ({ visible, onWidthChange, width, kit }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth + (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  // const { kit } = useKit();
  if (!kit?.types || !kit?.designs) return null

  const typesByName = kit.types.reduce(
    (acc, type) => {
      acc[type.name] = acc[type.name] || []
      acc[type.name].push(type)
      return acc
    },
    {} as Record<string, Type[]>
  )

  const designsByName = kit.designs.reduce(
    (acc, design) => {
      const nameKey = design.name
      acc[nameKey] = acc[nameKey] || []
      acc[nameKey].push(design)
      return acc
    },
    {} as Record<string, Design[]>
  )

  return (
    <div
      className={`absolute top-4 left-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-r-primary' : 'border-r'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="p-1">
          <TreeSection label="Types" defaultOpen={true}>
            {Object.entries(typesByName).map(([name, variants]) => (
              <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
                <div
                  className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                  style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}
                >
                  {variants.map((type) => (
                    <TypeAvatar key={`${type.name}-${type.variant}`} type={type} showHoverCard={true} />
                  ))}
                </div>
              </TreeNode>
            ))}
          </TreeSection>
          <TreeSection label="Designs" defaultOpen={true}>
            {Object.entries(designsByName).map(([name, designs]) => (
              <TreeNode key={name} label={name} collapsible={true} level={1} defaultOpen={false}>
                <div
                  className="grid grid-cols-[repeat(auto-fill,calc(var(--spacing)*8))] auto-rows-[calc(var(--spacing)*8)] justify-start gap-1 p-1"
                  style={{ paddingLeft: `${(1 + 1) * 1.25}rem` }}
                >
                  {designs.map((design) => (
                    <DesignAvatar
                      key={`${design.name}-${design.variant}-${design.view}`}
                      design={design}
                      showHoverCard={true}
                    />
                  ))}
                </div>
              </TreeNode>
            ))}
          </TreeSection>
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 right-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

interface ConsoleProps {
  visible: boolean
  leftPanelVisible: boolean
  rightPanelVisible: boolean
  leftPanelWidth?: number
  rightPanelWidth?: number
  height: number
  setHeight: (height: number) => void
}

const Console: FC<ConsoleProps> = ({
  visible,
  leftPanelVisible,
  rightPanelVisible,
  leftPanelWidth = 230,
  rightPanelWidth = 230,
  height,
  setHeight
}) => {
  if (!visible) return null

  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startY = e.clientY
    const startHeight = height

    const handleMouseMove = (e: MouseEvent) => {
      const newHeight = startHeight - (e.clientY - startY)
      if (newHeight >= 100 && newHeight <= 600) {
        setHeight(newHeight)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`absolute z-30 bg-background-level-2 text-foreground border ${isResizing || isResizeHovered ? 'border-t-primary' : ''}`}
      style={{
        left: leftPanelVisible ? `calc(${leftPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
        right: rightPanelVisible ? `calc(${rightPanelWidth}px + calc(var(--spacing) * 8))` : `calc(var(--spacing) * 4)`,
        bottom: `calc(var(--spacing) * 4)`,
        height: `${height}px`
      }}
    >
      <div
        className={`absolute top-0 left-0 right-0 h-1 cursor-ns-resize`}
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
      <ScrollArea className="h-full">
        <div className="font-semibold p-4">Console</div>
      </ScrollArea>
    </div>
  )
}

interface DetailsProps extends ResizablePanelProps { }

const Details: FC<DetailsProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div id="type-properties" className="p-4 space-y-4">
          <div className="space-y-2">
            <label className="text-sm">Name</label>
            <Input placeholder="Name" />
          </div>
          <div className="space-y-2">
            <label className="text-sm">Rotation</label>
            <Slider defaultValue={[33]} max={100} min={0} step={1} />
          </div>
          <div className="space-y-2">
            <label className="text-sm">Description</label>
            <Textarea />
          </div>
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

interface ChatProps extends ResizablePanelProps { }

const Chat: FC<ChatProps> = ({ visible, onWidthChange, width }) => {
  if (!visible) return null
  const [isResizeHovered, setIsResizeHovered] = useState(false)
  const [isResizing, setIsResizing] = useState(false)

  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault()
    setIsResizing(true)

    const startX = e.clientX
    const startWidth = width

    const handleMouseMove = (e: MouseEvent) => {
      const newWidth = startWidth - (e.clientX - startX)
      if (newWidth >= 150 && newWidth <= 500) {
        onWidthChange?.(newWidth)
      }
    }

    const handleMouseUp = () => {
      setIsResizing(false)
      document.removeEventListener('mousemove', handleMouseMove)
      document.removeEventListener('mouseup', handleMouseUp)
    }

    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`absolute top-4 right-4 bottom-4 z-20 bg-background-level-2 text-foreground border
                ${isResizing || isResizeHovered ? 'border-l-primary' : 'border-l'}`}
      style={{ width: `${width}px` }}
    >
      <ScrollArea className="h-full">
        <div className="font-semibold p-4">Chat</div>
        <div className="p-4 space-y-4">
          <Textarea />
        </div>
      </ScrollArea>
      <div
        className="absolute top-0 bottom-0 left-0 w-1 cursor-ew-resize"
        onMouseDown={handleMouseDown}
        onMouseEnter={() => setIsResizeHovered(true)}
        onMouseLeave={() => !isResizing && setIsResizeHovered(false)}
      />
    </div>
  )
}

//#endregion Panels

const DesignEditorCore: FC<DesignEditorProps> = (props) => {
  const { onToolbarChange, designId, onUndo, onRedo } = props

  // #region resolve state and dispatcher

  const isControlled = props.kit !== undefined

  const initialUncontrolledState: DesignEditorState = {
    kit: props.initialKit!,
    designId: props.designId,
    fileUrls: props.fileUrls,
    fullscreenPanel: FullscreenPanel.None,
    selection: props.initialSelection || { selectedPieceIds: [], selectedConnections: [] },
    designDiff: props.initialDesignDiff || {
      pieces: { added: [], removed: [], updated: [] },
      connections: { added: [], removed: [], updated: [] }
    }
  }
  const uncontrolledDesignEditorReducer = (
    state: DesignEditorState,
    action: { type: DesignEditorAction; payload: any }
  ): DesignEditorState => {
    const currentDesign = findDesign(state.kit, state.designId)

    // Helper function to update a design in the kit
    const updateDesignInKit = (updatedDesign: Design): DesignEditorState => {
      const updatedDesigns = (state.kit.designs || []).map((d: Design) =>
        sameDesign(d, currentDesign) ? updatedDesign : d
      )
      return { ...state, kit: { ...state.kit, designs: updatedDesigns } }
    }

    switch (action.type) {
      case DesignEditorAction.SetDesign:
        return updateDesignInKit(action.payload)

      // Design piece manipulation
      case DesignEditorAction.AddPieceToDesign:
        return updateDesignInKit(addPieceToDesign(currentDesign, action.payload))

      case DesignEditorAction.AddPiecesToDesign:
        return updateDesignInKit(addPiecesToDesign(currentDesign, action.payload))

      case DesignEditorAction.RemovePieceFromDesign:
        return updateDesignInKit(removePieceFromDesign(currentDesign, action.payload))

      case DesignEditorAction.RemovePiecesFromDesign:
        return updateDesignInKit(removePiecesFromDesign(currentDesign, action.payload))

      case DesignEditorAction.SetPieceInDesign:
        return updateDesignInKit(setPieceInDesign(currentDesign, action.payload))

      case DesignEditorAction.SetPiecesInDesign:
        return updateDesignInKit(setPiecesInDesign(currentDesign, action.payload))

      // Design connection manipulation
      case DesignEditorAction.AddConnectionToDesign:
        return updateDesignInKit(addConnectionToDesign(currentDesign, action.payload))

      case DesignEditorAction.AddConnectionsToDesign:
        return updateDesignInKit(addConnectionsToDesign(currentDesign, action.payload))

      case DesignEditorAction.RemoveConnectionFromDesign:
        return updateDesignInKit(removeConnectionFromDesign(currentDesign, action.payload))

      case DesignEditorAction.RemoveConnectionsFromDesign:
        return updateDesignInKit(removeConnectionsFromDesign(currentDesign, action.payload))

      case DesignEditorAction.SetConnectionInDesign:
        return updateDesignInKit(setConnectionInDesign(currentDesign, action.payload))

      case DesignEditorAction.SetConnectionsInDesign:
        return updateDesignInKit(setConnectionsInDesign(currentDesign, action.payload))

      // DesignDiff manipulation
      case DesignEditorAction.SetDesignDiff:
        return { ...state, designDiff: action.payload }

      case DesignEditorAction.AddPieceToDesignDiff:
        return { ...state, designDiff: addPieceToDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.AddPiecesToDesignDiff:
        return { ...state, designDiff: addPiecesToDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.RemovePieceFromDesignDiff:
        return { ...state, designDiff: removePieceFromDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.RemovePiecesFromDesignDiff:
        return { ...state, designDiff: removePiecesFromDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.SetPieceInDesignDiff:
        return { ...state, designDiff: setPieceInDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.SetPiecesInDesignDiff:
        return { ...state, designDiff: setPiecesInDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.AddConnectionToDesignDiff:
        return { ...state, designDiff: addConnectionToDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.AddConnectionsToDesignDiff:
        return { ...state, designDiff: addConnectionsToDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.RemoveConnectionFromDesignDiff:
        return { ...state, designDiff: removeConnectionFromDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.RemoveConnectionsFromDesignDiff:
        return { ...state, designDiff: removeConnectionsFromDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.SetConnectionInDesignDiff:
        return { ...state, designDiff: setConnectionInDesignDiff(state.designDiff, action.payload) }

      case DesignEditorAction.SetConnectionsInDesignDiff:
        return { ...state, designDiff: setConnectionsInDesignDiff(state.designDiff, action.payload) }

      // Selection manipulation
      case DesignEditorAction.SetSelection:
        return { ...state, selection: action.payload }

      case DesignEditorAction.AddPieceToSelection:
        return { ...state, selection: addPieceToSelection(state.selection, action.payload) }

      case DesignEditorAction.AddPiecesToSelection:
        return { ...state, selection: addPiecesToSelection(state.selection, action.payload) }

      case DesignEditorAction.RemovePieceFromSelection:
        return { ...state, selection: removePieceFromSelection(state.selection, action.payload) }

      case DesignEditorAction.RemovePiecesFromSelection:
        return { ...state, selection: removePiecesFromSelection(state.selection, action.payload) }

      case DesignEditorAction.AddConnectionToSelection:
        return { ...state, selection: addConnectionToSelection(state.selection, action.payload) }

      case DesignEditorAction.AddConnectionsToSelection:
        return { ...state, selection: addConnectionsToSelection(state.selection, action.payload) }

      case DesignEditorAction.RemoveConnectionFromSelection:
        return { ...state, selection: removeConnectionFromSelection(state.selection, action.payload) }

      case DesignEditorAction.RemoveConnectionsFromSelection:
        return { ...state, selection: removeConnectionsFromSelection(state.selection, action.payload) }

      // Other actions
      case DesignEditorAction.SetFullscreen:
        return { ...state, fullscreenPanel: action.payload }

      default:
        return state
    }
  }
  const [uncontrolledState, uncontrolledDispatch] = useReducer(
    uncontrolledDesignEditorReducer,
    initialUncontrolledState
  )

  const controlledState = useMemo(() => {
    return {
      kit: props.kit!,
      designId: props.designId,
      fileUrls: props.fileUrls,
      selection: props.selection,
      designDiff: props.designDiff,
      fullscreenPanel: FullscreenPanel.None
    } as DesignEditorState
  }, [props.kit, props.designId, props.fileUrls, props.selection, props.designDiff])
  const controlledDispatch = useCallback(
    (action: { type: DesignEditorAction; payload: any }) => {
      if (action.type === DesignEditorAction.SetDesign) props.onDesignChange?.(action.payload)
      if (action.type === DesignEditorAction.SetSelection) props.onSelectionChange?.(action.payload)
    },
    [props.onDesignChange, props.onSelectionChange]
  )

  // #endregion resolve state and dispatcher

  const state = isControlled ? controlledState : uncontrolledState
  const dispatch = isControlled ? controlledDispatch : uncontrolledDispatch

  if (!state.kit) return null

  const design = findDesign(state.kit, designId)
  if (!design) return null

  // #region Panels
  const [visiblePanels, setVisiblePanels] = useState<PanelToggles>({
    workbench: false,
    console: false,
    details: false,
    chat: false
  })
  const [workbenchWidth, setWorkbenchWidth] = useState(230)
  const [detailsWidth, setDetailsWidth] = useState(230)
  const [consoleHeight, setConsoleHeight] = useState(200)
  const [chatWidth, setChatWidth] = useState(230)

  const togglePanel = (panel: keyof PanelToggles) => {
    setVisiblePanels((prev) => {
      const newState = { ...prev }
      if (panel === 'chat' && !prev.chat) {
        newState.details = false
      }
      if (panel === 'details' && !prev.details) {
        newState.chat = false
      }
      newState[panel] = !prev[panel]
      return newState
    })
  }
  // #endregion Panels

  // #region Hotkeys
  useHotkeys('mod+j', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('workbench')
  })
  useHotkeys('mod+k', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('console')
  })
  useHotkeys('mod+l', (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('details')
  })
  useHotkeys(['mod+[', 'mod+semicolon', 'mod+ö'], (e) => {
    e.preventDefault()
    e.stopPropagation()
    togglePanel('chat')
  })

  useHotkeys('mod+a', (e) => {
    e.preventDefault()
    const allIds = design?.pieces?.map((p) => p.id_) || []
    dispatch({ type: DesignEditorAction.SetSelection, payload: { selectedPieceIds: allIds, selectedConnections: [] } })
  })
  useHotkeys('mod+i', (e) => {
    e.preventDefault()
    console.log('Invert selection')
  })
  useHotkeys('mod+d', (e) => {
    e.preventDefault()
    console.log('Select closest piece with same variant')
  })
  useHotkeys('mod+shift+d', (e) => {
    e.preventDefault()
    console.log('Select all pieces with same variant')
  })
  useHotkeys('mod+c', (e) => {
    e.preventDefault()
    navigator.clipboard.writeText(JSON.stringify(findDesign(state.kit, designId)))
  })

  useHotkeys('mod+v', (e) => {
    e.preventDefault()
    console.log('Paste')
  })
  useHotkeys('mod+x', (e) => {
    e.preventDefault()
    console.log('Cut selected')
  })
  useHotkeys('delete', (e) => {
    e.preventDefault()
    // TODO: Implement selection deletion
    console.log('Delete selected')
  })
  useHotkeys('mod+z', (e) => {
    // Swapped y and z for conventional undo/redo
    e.preventDefault()
    onUndo?.()
  })
  useHotkeys('mod+y', (e) => {
    e.preventDefault()
    onRedo?.()
  })
  useHotkeys('mod+w', (e) => {
    e.preventDefault()
    console.log('Close design')
  })
  // #endregion Hotkeys

  const rightPanelVisible = visiblePanels.details || visiblePanels.chat

  const designEditorToolbar = (
    <ToggleGroup
      type="multiple"
      value={Object.entries(visiblePanels)
        .filter(([_, isVisible]) => isVisible)
        .map(([key]) => key)}
      onValueChange={(values) => {
        Object.keys(visiblePanels).forEach((key) => {
          const isCurrentlyVisible = visiblePanels[key as keyof PanelToggles]
          const shouldBeVisible = values.includes(key)
          if (isCurrentlyVisible !== shouldBeVisible) {
            togglePanel(key as keyof PanelToggles)
          }
        })
      }}
    >
      <ToggleGroupItem value="workbench" tooltip="Workbench" hotkey="⌘J">
        <Wrench />
      </ToggleGroupItem>
      <ToggleGroupItem value="console" tooltip="Console" hotkey="⌘K">
        <Terminal />
      </ToggleGroupItem>
      <ToggleGroupItem value="details" tooltip="Details" hotkey="⌘L">
        <Info />
      </ToggleGroupItem>
      <ToggleGroupItem value="chat" tooltip="Chat" hotkey="⌘[">
        <MessageCircle />
      </ToggleGroupItem>
    </ToggleGroup>
  )

  useEffect(() => {
    onToolbarChange(designEditorToolbar)
    return () => onToolbarChange(null)
  }, [visiblePanels])

  const { screenToFlowPosition } = useReactFlow()
  const [activeDraggedType, setActiveDraggedType] = useState<Type | null>(null)
  const [activeDraggedDesign, setActiveDraggedDesign] = useState<Design | null>(null)

  // #region Drag and Drop

  const onDragStart = (event: DragStartEvent) => {
    const { active } = event
    const id = active.id.toString()
    if (id.startsWith('type-')) {
      const [_, name, variant] = id.split('-')
      // Normalize variants so that undefined, null and empty string are treated the same
      const normalizeVariant = (v: string | undefined | null) => v ?? ''
      const type = state.kit?.types?.find(
        (t: Type) => t.name === name && normalizeVariant(t.variant) === normalizeVariant(variant)
      )
      setActiveDraggedType(type || null)
    } else if (id.startsWith('design-')) {
      const [_, name, variant, view] = id.split('-')
      const draggedDesignId: DesignId = { name, variant: variant || undefined, view: view || undefined }
      const draggedDesign = findDesign(state.kit, draggedDesignId)
      setActiveDraggedDesign(draggedDesign || null)
    }
  }

  const onDragEnd = (event: DragEndEvent) => {
    const { over } = event
    if (over?.id === 'diagram') {
      if (!(event.activatorEvent instanceof PointerEvent)) {
        return
      }
      if (activeDraggedType) {
        const { x, y } = screenToFlowPosition({
          x: event.activatorEvent.clientX + event.delta.x,
          y: event.activatorEvent.clientY + event.delta.y
        })
        const piece: Piece = {
          id_: Generator.randomId(),
          type: {
            name: activeDraggedType.name,
            variant: activeDraggedType.variant || undefined
          },
          plane: {
            origin: { x: 0, y: 0, z: 0 },
            xAxis: { x: 1, y: 0, z: 0 },
            yAxis: { x: 0, y: 1, z: 0 }
          },
          center: { x: x / ICON_WIDTH - 0.5, y: -y / ICON_WIDTH + 0.5 }
        }
        dispatch({
          type: DesignEditorAction.SetDesign,
          payload: { ...design, pieces: [...(design.pieces || []), piece] }
        })
      } else if (activeDraggedDesign) {
        throw new Error('Not implemented')
      }
    }
    setActiveDraggedType(null)
    setActiveDraggedDesign(null)
  }

  // #endregion Drag and Drop

  return (
    <DndContext onDragStart={onDragStart} onDragEnd={onDragEnd}>
      <div className="canvas flex-1 relative">
        <div id="sketchpad-edgeless" className="h-full">
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel
              defaultSize={state.fullscreenPanel === FullscreenPanel.Diagram ? 100 : 50}
              className={`${state.fullscreenPanel === FullscreenPanel.Model ? 'hidden' : 'block'}`}
              onDoubleClick={() =>
                dispatch({ type: DesignEditorAction.SetFullscreen, payload: FullscreenPanel.Diagram })
              }
            >
              <Diagram designEditorState={state} designEditorDispatcher={dispatch} />
            </ResizablePanel>
            <ResizableHandle
              className={`border-r ${state.fullscreenPanel !== FullscreenPanel.None ? 'hidden' : 'block'}`}
            />
            <ResizablePanel
              defaultSize={state.fullscreenPanel === FullscreenPanel.Model ? 100 : 50}
              className={`${state.fullscreenPanel === FullscreenPanel.Diagram ? 'hidden' : 'block'}`}
              onDoubleClick={() => dispatch({ type: DesignEditorAction.SetFullscreen, payload: FullscreenPanel.Model })}
            >
              <Model designEditorState={state} designEditorDispatcher={dispatch} />
            </ResizablePanel>
          </ResizablePanelGroup>
        </div>
        <Workbench
          visible={visiblePanels.workbench}
          onWidthChange={setWorkbenchWidth}
          width={workbenchWidth}
          kit={state.kit!}
        />
        <Details visible={visiblePanels.details} onWidthChange={setDetailsWidth} width={detailsWidth} />
        <Console
          visible={visiblePanels.console}
          leftPanelVisible={visiblePanels.workbench}
          rightPanelVisible={rightPanelVisible}
          leftPanelWidth={workbenchWidth}
          rightPanelWidth={detailsWidth}
          height={consoleHeight}
          setHeight={setConsoleHeight}
        />
        <Chat visible={visiblePanels.chat} onWidthChange={setChatWidth} width={chatWidth} />
        {createPortal(
          <DragOverlay>
            {activeDraggedType && <TypeAvatar type={activeDraggedType} />}
            {activeDraggedDesign && <DesignAvatar design={activeDraggedDesign} />}
          </DragOverlay>,
          document.body
        )}
      </div>
    </DndContext>
  )
}

interface ControlledDesignEditorProps {
  kit: Kit
  selection: DesignEditorSelection
  designDiff: DesignDiff
  onDesignChange: (design: Design) => void
  onDesignDiffChange: (designDiff: DesignDiff) => void
  onSelectionChange: (selection: DesignEditorSelection) => void
  onUndo: () => void
  onRedo: () => void
}

interface UncontrolledDesignEditorProps {
  initialKit: Kit
  initialDesignDiff: DesignDiff
  initialSelection?: DesignEditorSelection
}

interface DesignEditorProps extends ControlledDesignEditorProps, UncontrolledDesignEditorProps {
  designId: DesignId
  fileUrls: Map<string, string>
  onToolbarChange: (toolbar: ReactNode) => void
  mode?: Mode
  layout?: Layout
  theme?: Theme
  setLayout?: (layout: Layout) => void
  setTheme?: (theme: Theme) => void
  onWindowEvents?: {
    minimize: () => void
    maximize: () => void
    close: () => void
  }
}

const DesignEditor: FC<DesignEditorProps> = ({
  mode,
  layout,
  theme,
  setLayout,
  setTheme,
  onWindowEvents,
  designId,
  kit,
  designDiff,
  selection,
  initialKit,
  initialDesignDiff,
  initialSelection,
  fileUrls,
  onDesignChange,
  onDesignDiffChange,
  onSelectionChange,
  onUndo,
  onRedo
}) => {
  const [toolbarContent, setToolbarContent] = useState<ReactNode>(null)

  return (
    <div key={`layout-${layout}`} className="h-full w-full flex flex-col bg-background text-foreground">
      <Navbar
        mode={mode}
        toolbarContent={toolbarContent}
        layout={layout}
        theme={theme}
        setLayout={setLayout}
        setTheme={setTheme}
        onWindowEvents={onWindowEvents}
      />
      <ReactFlowProvider>
        <DesignEditorCore
          kit={kit}
          designId={designId}
          fileUrls={fileUrls}
          designDiff={designDiff}
          selection={selection}
          initialKit={initialKit}
          initialDesignDiff={initialDesignDiff}
          initialSelection={initialSelection}
          onDesignChange={onDesignChange!}
          onDesignDiffChange={onDesignDiffChange!}
          onSelectionChange={onSelectionChange!}
          onUndo={onUndo}
          onRedo={onRedo}
          onToolbarChange={setToolbarContent}
        />
      </ReactFlowProvider>
    </div>
  )
}

export default DesignEditor

//#endregion
