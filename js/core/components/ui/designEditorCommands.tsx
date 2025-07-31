// Design Editor Commands
// Centralized command definitions for the Design Editor

import {
    Design,
    addConnectionToDesign,
    findDesignInKit,
    flattenDesign,
    removePiecesAndConnectionsFromDesign,
    setPieceInDesign
} from '@semio/js'
import { orientDesign } from '../../semio'
import { Command, commandRegistry } from './Console'

// Helper functions for commands
const addPieceToDesignHelper = (design: Design, piece: any): Design => ({
    ...design,
    pieces: [...(design.pieces || []), piece]
})

// Selection utility functions (taken from DesignEditor.tsx)
const selectAll = (design: Design): any => ({
    selectedPieceIds: design.pieces?.map((p: any) => p.id_) || [],
    selectedConnections: design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || [],
    selectedPiecePortId: undefined
})

const deselectAll = (selection: any): any => ({
    selectedPieceIds: [],
    selectedConnections: [],
    selectedPiecePortId: undefined
})

const invertSelection = (selection: any, design: Design): any => {
    const allPieceIds = design.pieces?.map((p: any) => p.id_) || []
    const allConnections = design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || []
    const newSelectedPieceIds = allPieceIds.filter(id => !selection.selectedPieceIds.includes(id))
    const newSelectedConnections = allConnections.filter(conn => {
        return !selection.selectedConnections.some((selected: any) =>
            selected.connectingPieceId === conn.connectingPieceId && selected.connectedPieceId === conn.connectedPieceId
        )
    })
    return ({ selectedPieceIds: newSelectedPieceIds, selectedConnections: newSelectedConnections, selectedPiecePortId: undefined })
}

const selectAllPieces = (design: Design): any => ({
    selectedPieceIds: design.pieces?.map((p: any) => p.id_) || [],
    selectedConnections: [],
    selectedPiecePortId: undefined
})

const selectAllConnections = (design: Design): any => ({
    selectedPieceIds: [],
    selectedConnections: design.connections?.map((c: any) => ({ connectedPieceId: c.connected.piece.id_, connectingPieceId: c.connecting.piece.id_ })) || [],
    selectedPiecePortId: undefined
})

const selectConnected = (design: Design, selection: any): any => {
    const connectedPieceIds = new Set<string>()
    selection.selectedPieceIds.forEach((pieceId: string) => {
        design.connections?.forEach((conn: any) => {
            if (conn.connecting.piece.id_ === pieceId) {
                connectedPieceIds.add(conn.connected.piece.id_)
            }
            if (conn.connected.piece.id_ === pieceId) {
                connectedPieceIds.add(conn.connecting.piece.id_)
            }
        })
    })
    return {
        ...selection,
        selectedPieceIds: [...new Set([...selection.selectedPieceIds, ...Array.from(connectedPieceIds)])]
    }
}

const selectDisconnected = (design: Design): any => {
    const connectedPieceIds = new Set<string>()
    design.connections?.forEach((conn: any) => {
        connectedPieceIds.add(conn.connecting.piece.id_)
        connectedPieceIds.add(conn.connected.piece.id_)
    })
    const disconnectedPieceIds = design.pieces?.filter((p: any) => !connectedPieceIds.has(p.id_)).map((p: any) => p.id_) || []
    return {
        selectedPieceIds: disconnectedPieceIds,
        selectedConnections: [],
        selectedPiecePortId: undefined
    }
}

const selectPiecesOfType = (design: Design, typeId: any): any => {
    const matchingPieceIds = design.pieces?.filter((p: any) =>
        p.type.name === typeId.name && (typeId.variant === undefined || p.type.variant === typeId.variant)
    ).map((p: any) => p.id_) || []
    return {
        selectedPieceIds: matchingPieceIds,
        selectedConnections: [],
        selectedPiecePortId: undefined
    }
}

export const designEditorCommands: Command[] = [
    // === PIECE COMMANDS ===
    {
        id: 'add-piece',
        name: 'Add Piece',
        icon: '➕',
        description: 'Add a new piece to the design',
        parameters: [
            { name: 'type', type: 'TypeId', description: 'Type of the piece', required: true },
            { name: 'center', type: 'string', description: 'Center coordinates (x,y)', defaultValue: '0,0' },
            { name: 'fixed', type: 'boolean', description: 'Fix piece at position', defaultValue: false }
        ],
        execute: async (context, payload) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const center = payload.center ? payload.center.split(',').map((n: string) => parseFloat(n.trim())) : [0, 0]
            const plane = payload.fixed ? {
                origin: { x: center[0], y: center[1], z: 0 },
                xAxis: { x: 1, y: 0, z: 0 },
                yAxis: { x: 0, y: 1, z: 0 }
            } : undefined

            const piece = {
                id_: `piece-${Date.now()}`,
                type: payload.type,
                center: { x: center[0], y: center[1] },
                plane
            }

            return {
                design: addPieceToDesignHelper(design, piece),
                content: `✅ Added piece: ${payload.type.name}`
            }
        }
    },

    {
        id: 'duplicate-selected',
        name: 'Duplicate Selected',
        icon: '📋',
        description: 'Duplicate the selected pieces',
        parameters: [
            { name: 'offset', type: 'string', description: 'Offset coordinates (x,y)', defaultValue: '1,0' }
        ],
        hotkey: 'mod+d',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            const offset = payload.offset ? payload.offset.split(',').map((n: string) => parseFloat(n.trim())) : [1, 0]

            let newDesign = design
            const newPieceIds: string[] = []

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const newPiece = {
                        ...piece,
                        id_: `piece-${Date.now()}-${Math.random()}`,
                        center: {
                            x: (piece.center?.x || 0) + offset[0],
                            y: (piece.center?.y || 0) + offset[1]
                        }
                    }
                    newDesign = addPieceToDesignHelper(newDesign, newPiece)
                    newPieceIds.push(newPiece.id_)
                }
            }

            return {
                design: newDesign,
                selection: { ...selection, selectedPieceIds: newPieceIds },
                content: `✅ Duplicated ${newPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'fix-selected-pieces',
        name: 'Fix Selected Pieces',
        icon: '📌',
        description: 'Fix the selected pieces at their current positions',
        parameters: [],
        hotkey: 'f',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            let newDesign = design

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece && !piece.plane) {
                    const updatedPiece = {
                        ...piece,
                        plane: {
                            origin: { x: piece.center?.x || 0, y: piece.center?.y || 0, z: 0 },
                            xAxis: { x: 1, y: 0, z: 0 },
                            yAxis: { x: 0, y: 1, z: 0 }
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Fixed ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'unfix-selected-pieces',
        name: 'Unfix Selected Pieces',
        icon: '📌',
        description: 'Unfix the selected pieces (remove their plane)',
        parameters: [],
        hotkey: 'shift+f',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            let newDesign = design

            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece && piece.plane) {
                    const updatedPiece = { ...piece, plane: undefined }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Unfixed ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    // === DELETION COMMANDS ===
    {
        id: 'delete-selected',
        name: 'Delete Selected',
        icon: '🗑️',
        description: 'Delete the selected pieces and connections',
        parameters: [],
        hotkey: 'delete',
        execute: async (context) => {
            const { kit, designId, selection } = context
            const selectedPieces = selection.selectedPieceIds.map((id: string) => ({ id_: id }))
            const selectedConnections = selection.selectedConnections.map((conn: any) => ({
                connecting: { piece: { id_: conn.connectingPieceId } },
                connected: { piece: { id_: conn.connectedPieceId } }
            }))
            return {
                design: removePiecesAndConnectionsFromDesign(kit, designId, selectedPieces, selectedConnections),
                selection: deselectAll(selection),
                content: `✅ Deleted ${selectedPieces.length} pieces and ${selectedConnections.length} connections`
            }
        }
    },

    {
        id: 'delete-all-pieces',
        name: 'Delete All Pieces',
        icon: '🗑️',
        description: 'Delete all pieces from the design',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: { ...design, pieces: [], connections: [] },
                selection: deselectAll(context.selection),
                content: `✅ Deleted all pieces and connections`
            }
        }
    },

    // === SELECTION COMMANDS ===
    {
        id: 'select-all',
        name: 'Select All',
        icon: '🔘',
        description: 'Select all pieces and connections',
        parameters: [],
        hotkey: 'mod+a',
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAll(design),
                content: `✅ Selected all pieces and connections`
            }
        }
    },

    {
        id: 'deselect-all',
        name: 'Deselect All',
        icon: '⭕',
        description: 'Deselect all pieces and connections',
        parameters: [],
        hotkey: 'escape',
        editorOnly: true,
        execute: async (context) => {
            return {
                selection: deselectAll(context.selection),
                content: `✅ Deselected all`
            }
        }
    },

    {
        id: 'invert-selection',
        name: 'Invert Selection',
        icon: '🔄',
        description: 'Invert the current selection',
        parameters: [],
        hotkey: 'mod+i',
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: invertSelection(context.selection, design),
                content: `✅ Inverted selection`
            }
        }
    },

    {
        id: 'select-all-pieces',
        name: 'Select All Pieces',
        icon: '🎯',
        description: 'Select all pieces (not connections)',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAllPieces(design),
                content: `✅ Selected all pieces`
            }
        }
    },

    {
        id: 'select-all-connections',
        name: 'Select All Connections',
        icon: '🔗',
        description: 'Select all connections (not pieces)',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectAllConnections(design),
                content: `✅ Selected all connections`
            }
        }
    },

    {
        id: 'select-connected',
        name: 'Select Connected',
        icon: '🔗',
        description: 'Select all pieces connected to the current selection',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectConnected(design, context.selection),
                content: `✅ Selected connected pieces`
            }
        }
    },

    {
        id: 'select-disconnected',
        name: 'Select Disconnected',
        icon: '⚡',
        description: 'Select all pieces not connected to anything',
        parameters: [],
        editorOnly: true,
        execute: async (context) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectDisconnected(design),
                content: `✅ Selected disconnected pieces`
            }
        }
    },

    {
        id: 'select-by-type',
        name: 'Select by Type',
        icon: '🏷️',
        description: 'Select all pieces of a specific type',
        parameters: [
            { name: 'type', type: 'TypeId', description: 'Type to select', required: true }
        ],
        execute: async (context, payload) => {
            const design = findDesignInKit(context.kit, context.designId)
            return {
                selection: selectPiecesOfType(design, payload.type),
                content: `✅ Selected pieces of type: ${payload.type.name}`
            }
        }
    },

    // === DESIGN MANIPULATION COMMANDS ===
    {
        id: 'flatten-design',
        name: 'Flatten Design',
        icon: '🏗️',
        description: 'Flatten the design by fixing all pieces at calculated positions',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: flattenDesign(kit, designId),
                content: `✅ Design flattened`
            }
        }
    },

    {
        id: 'orient-design',
        name: 'Orient Design',
        icon: '🧭',
        description: 'Orient the design based on the connections',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: orientDesign(design),
                content: `✅ Design oriented`
            }
        }
    },

    {
        id: 'center-design',
        name: 'Center Design',
        icon: '📐',
        description: 'Center all pieces around the origin',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            if (!design.pieces?.length) {
                return { content: `⚠️ No pieces to center` }
            }

            // Calculate centroid
            const totalX = design.pieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = design.pieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / design.pieces.length
            const centroidY = totalY / design.pieces.length

            // Offset all pieces
            const centeredPieces = design.pieces.map(piece => ({
                ...piece,
                center: {
                    x: (piece.center?.x || 0) - centroidX,
                    y: (piece.center?.y || 0) - centroidY
                }
            }))

            return {
                design: { ...design, pieces: centeredPieces },
                content: `✅ Design centered`
            }
        }
    },

    // === CONNECTION COMMANDS ===
    {
        id: 'add-connection',
        name: 'Add Connection',
        icon: '🔗',
        description: 'Add a connection between two pieces',
        parameters: [
            { name: 'connectingPiece', type: 'PieceId', description: 'Connecting piece ID', required: true },
            { name: 'connectedPiece', type: 'PieceId', description: 'Connected piece ID', required: true },
            { name: 'connectingPort', type: 'PortId', description: 'Connecting port ID', required: false },
            { name: 'connectedPort', type: 'PortId', description: 'Connected port ID', required: false }
        ],
        execute: async (context, payload) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const connection = {
                id_: `connection-${Date.now()}`,
                connecting: {
                    piece: { id_: payload.connectingPiece },
                    port: payload.connectingPort ? { id_: payload.connectingPort } : { id_: undefined }
                },
                connected: {
                    piece: { id_: payload.connectedPiece },
                    port: payload.connectedPort ? { id_: payload.connectedPort } : { id_: undefined }
                }
            }

            return {
                design: addConnectionToDesign(design, connection),
                content: `✅ Added connection`
            }
        }
    },

    {
        id: 'delete-all-connections',
        name: 'Delete All Connections',
        icon: '⛓️‍💥',
        description: 'Delete all connections from the design',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)
            return {
                design: { ...design, connections: [] },
                selection: { ...context.selection, selectedConnections: [] },
                content: `✅ Deleted all connections`
            }
        }
    },

    // === GRID AND ALIGNMENT COMMANDS ===
    {
        id: 'align-pieces-horizontal',
        name: 'Align Pieces Horizontally',
        icon: '⚖️',
        description: 'Align selected pieces horizontally',
        parameters: [
            {
                name: 'position', type: 'select', description: 'Alignment position', options: [
                    { value: 'top', label: 'Top' },
                    { value: 'center', label: 'Center' },
                    { value: 'bottom', label: 'Bottom' }
                ], defaultValue: 'center'
            }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 2) {
                return { content: `⚠️ Select at least 2 pieces to align` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const yPositions = selectedPieces.map(p => p.center?.y || 0)

            let targetY: number
            switch (payload.position) {
                case 'top': targetY = Math.max(...yPositions); break
                case 'bottom': targetY = Math.min(...yPositions); break
                default: targetY = yPositions.reduce((sum, y) => sum + y, 0) / yPositions.length
            }

            let newDesign = design
            for (const piece of selectedPieces) {
                const updatedPiece = {
                    ...piece,
                    center: { ...piece.center, x: piece.center?.x || 0, y: targetY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Aligned ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'align-pieces-vertical',
        name: 'Align Pieces Vertically',
        icon: '⚖️',
        description: 'Align selected pieces vertically',
        parameters: [
            {
                name: 'position', type: 'select', description: 'Alignment position', options: [
                    { value: 'left', label: 'Left' },
                    { value: 'center', label: 'Center' },
                    { value: 'right', label: 'Right' }
                ], defaultValue: 'center'
            }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 2) {
                return { content: `⚠️ Select at least 2 pieces to align` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const xPositions = selectedPieces.map(p => p.center?.x || 0)

            let targetX: number
            switch (payload.position) {
                case 'left': targetX = Math.min(...xPositions); break
                case 'right': targetX = Math.max(...xPositions); break
                default: targetX = xPositions.reduce((sum, x) => sum + x, 0) / xPositions.length
            }

            let newDesign = design
            for (const piece of selectedPieces) {
                const updatedPiece = {
                    ...piece,
                    center: { ...piece.center, x: targetX, y: piece.center?.y || 0 }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Aligned ${selectedPieces.length} pieces vertically`
            }
        }
    },

    {
        id: 'distribute-pieces-horizontal',
        name: 'Distribute Pieces Horizontally',
        icon: '📏',
        description: 'Distribute selected pieces evenly horizontally',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 3) {
                return { content: `⚠️ Select at least 3 pieces to distribute` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            selectedPieces.sort((a, b) => (a.center?.x || 0) - (b.center?.x || 0))

            const minX = selectedPieces[0].center?.x || 0
            const maxX = selectedPieces[selectedPieces.length - 1].center?.x || 0
            const spacing = (maxX - minX) / (selectedPieces.length - 1)

            let newDesign = design
            for (let i = 1; i < selectedPieces.length - 1; i++) {
                const piece = selectedPieces[i]
                const updatedPiece = {
                    ...piece,
                    center: { x: minX + i * spacing, y: piece.center?.y || 0 }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Distributed ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'distribute-pieces-vertical',
        name: 'Distribute Pieces Vertically',
        icon: '📏',
        description: 'Distribute selected pieces evenly vertically',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length < 3) {
                return { content: `⚠️ Select at least 3 pieces to distribute` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            selectedPieces.sort((a, b) => (a.center?.y || 0) - (b.center?.y || 0))

            const minY = selectedPieces[0].center?.y || 0
            const maxY = selectedPieces[selectedPieces.length - 1].center?.y || 0
            const spacing = (maxY - minY) / (selectedPieces.length - 1)

            let newDesign = design
            for (let i = 1; i < selectedPieces.length - 1; i++) {
                const piece = selectedPieces[i]
                const updatedPiece = {
                    ...piece,
                    center: { x: piece.center?.x || 0, y: minY + i * spacing }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Distributed ${selectedPieces.length} pieces vertically`
            }
        }
    },

    // === TRANSFORM COMMANDS ===
    {
        id: 'move-selected',
        name: 'Move Selected',
        icon: '🏃',
        description: 'Move selected pieces by offset',
        parameters: [
            { name: 'offset', type: 'string', description: 'Move offset (x,y)', defaultValue: '0,0', required: true }
        ],
        hotkey: 'mod+m',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)
            const offset = payload.offset.split(',').map((n: string) => parseFloat(n.trim()))

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to move` }
            }

            let newDesign = design
            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const updatedPiece = {
                        ...piece,
                        center: {
                            x: (piece.center?.x || 0) + offset[0],
                            y: (piece.center?.y || 0) + offset[1]
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Moved ${selection.selectedPieceIds.length} pieces`
            }
        }
    },

    {
        id: 'rotate-selected',
        name: 'Rotate Selected',
        icon: '🔄',
        description: 'Rotate selected pieces around their centroid',
        parameters: [
            { name: 'angle', type: 'number', description: 'Rotation angle in degrees', defaultValue: 90, required: true }
        ],
        hotkey: 'mod+r',
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to rotate` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / selectedPieces.length
            const centroidY = totalY / selectedPieces.length

            const angle = (payload.angle * Math.PI) / 180
            const cos = Math.cos(angle)
            const sin = Math.sin(angle)

            let newDesign = design
            for (const piece of selectedPieces) {
                const relX = (piece.center?.x || 0) - centroidX
                const relY = (piece.center?.y || 0) - centroidY
                const newX = centroidX + relX * cos - relY * sin
                const newY = centroidY + relX * sin + relY * cos

                const updatedPiece = {
                    ...piece,
                    center: { x: newX, y: newY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Rotated ${selectedPieces.length} pieces by ${payload.angle}°`
            }
        }
    },

    {
        id: 'scale-selected',
        name: 'Scale Selected',
        icon: '🔍',
        description: 'Scale selected pieces around their centroid',
        parameters: [
            { name: 'factor', type: 'number', description: 'Scale factor', defaultValue: 1.5, required: true }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to scale` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidX = totalX / selectedPieces.length
            const centroidY = totalY / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const relX = (piece.center?.x || 0) - centroidX
                const relY = (piece.center?.y || 0) - centroidY
                const newX = centroidX + relX * payload.factor
                const newY = centroidY + relY * payload.factor

                const updatedPiece = {
                    ...piece,
                    center: { x: newX, y: newY }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Scaled ${selectedPieces.length} pieces by ${payload.factor}x`
            }
        }
    },

    // === ANALYSIS COMMANDS ===
    {
        id: 'analyze-design',
        name: 'Analyze Design',
        icon: '🔍',
        description: 'Show design statistics and analysis',
        parameters: [],
        execute: async (context) => {
            const { kit, designId } = context
            const design = findDesignInKit(kit, designId)

            const pieceCount = design.pieces?.length || 0
            const connectionCount = design.connections?.length || 0

            const typeCount = new Map<string, number>()
            design.pieces?.forEach(piece => {
                const typeName = piece.type.name
                typeCount.set(typeName, (typeCount.get(typeName) || 0) + 1)
            })

            const fixedPieces = design.pieces?.filter(p => p.plane) || []
            const disconnectedPieces = design.pieces?.filter(piece => {
                return !design.connections?.some(conn =>
                    conn.connecting.piece.id_ === piece.id_ || conn.connected.piece.id_ === piece.id_
                )
            }) || []

            const typesList = Array.from(typeCount.entries())
                .map(([type, count]) => `  • ${type}: ${count}`)
                .join('\n')

            return {
                content: `📊 Design Analysis:
• Pieces: ${pieceCount}
• Connections: ${connectionCount}
• Fixed pieces: ${fixedPieces.length}
• Disconnected pieces: ${disconnectedPieces.length}

🏷️ Types used:
${typesList}`
            }
        }
    },

    {
        id: 'randomize-positions',
        name: 'Randomize Positions',
        icon: '🎲',
        description: 'Randomize positions of selected pieces',
        parameters: [
            { name: 'range', type: 'number', description: 'Randomization range', defaultValue: 5 }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to randomize` }
            }

            let newDesign = design
            for (const pieceId of selection.selectedPieceIds) {
                const piece = design.pieces?.find(p => p.id_ === pieceId)
                if (piece) {
                    const updatedPiece = {
                        ...piece,
                        center: {
                            x: (piece.center?.x || 0) + (Math.random() - 0.5) * payload.range * 2,
                            y: (piece.center?.y || 0) + (Math.random() - 0.5) * payload.range * 2
                        }
                    }
                    newDesign = setPieceInDesign(newDesign, updatedPiece)
                }
            }

            return {
                design: newDesign,
                content: `✅ Randomized ${selection.selectedPieceIds.length} piece positions`
            }
        }
    },

    {
        id: 'create-grid-layout',
        name: 'Create Grid Layout',
        icon: '📋',
        description: 'Arrange selected pieces in a grid layout',
        parameters: [
            { name: 'columns', type: 'number', description: 'Number of columns', defaultValue: 3, required: true },
            { name: 'spacing', type: 'number', description: 'Spacing between pieces', defaultValue: 2, required: true }
        ],
        execute: async (context, payload) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to arrange` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const { columns, spacing } = payload

            let newDesign = design
            selectedPieces.forEach((piece, index) => {
                const row = Math.floor(index / columns)
                const col = index % columns
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: col * spacing,
                        y: row * spacing
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            })

            return {
                design: newDesign,
                content: `✅ Arranged ${selectedPieces.length} pieces in ${columns}-column grid`
            }
        }
    },

    {
        id: 'mirror-selected-horizontal',
        name: 'Mirror Selected Horizontally',
        icon: '🪞',
        description: 'Mirror selected pieces horizontally around their centroid',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to mirror` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalX = selectedPieces.reduce((sum, p) => sum + (p.center?.x || 0), 0)
            const centroidX = totalX / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const distanceFromCenter = (piece.center?.x || 0) - centroidX
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: centroidX - distanceFromCenter,
                        y: piece.center?.y || 0
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Mirrored ${selectedPieces.length} pieces horizontally`
            }
        }
    },

    {
        id: 'mirror-selected-vertical',
        name: 'Mirror Selected Vertically',
        icon: '🪞',
        description: 'Mirror selected pieces vertically around their centroid',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to mirror` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []

            // Calculate centroid
            const totalY = selectedPieces.reduce((sum, p) => sum + (p.center?.y || 0), 0)
            const centroidY = totalY / selectedPieces.length

            let newDesign = design
            for (const piece of selectedPieces) {
                const distanceFromCenter = (piece.center?.y || 0) - centroidY
                const updatedPiece = {
                    ...piece,
                    center: {
                        x: piece.center?.x || 0,
                        y: centroidY - distanceFromCenter
                    }
                }
                newDesign = setPieceInDesign(newDesign, updatedPiece)
            }

            return {
                design: newDesign,
                content: `✅ Mirrored ${selectedPieces.length} pieces vertically`
            }
        }
    },

    // === EXPORT/IMPORT COMMANDS ===
    {
        id: 'export-selected-to-clipboard',
        name: 'Export Selected to Clipboard',
        icon: '📋',
        description: 'Copy selected pieces to clipboard as JSON',
        parameters: [],
        execute: async (context) => {
            const { kit, designId, selection } = context
            const design = findDesignInKit(kit, designId)

            if (selection.selectedPieceIds.length === 0) {
                return { content: `⚠️ No pieces selected to export` }
            }

            const selectedPieces = design.pieces?.filter(p => selection.selectedPieceIds.includes(p.id_)) || []
            const selectedConnections = design.connections?.filter(conn =>
                selection.selectedConnections.some((selConn: any) =>
                    selConn.connectingPieceId === conn.connecting.piece.id_ &&
                    selConn.connectedPieceId === conn.connected.piece.id_
                )
            ) || []

            const exportData = {
                pieces: selectedPieces,
                connections: selectedConnections
            }

            try {
                await navigator.clipboard.writeText(JSON.stringify(exportData, null, 2))
                return {
                    content: `✅ Exported ${selectedPieces.length} pieces and ${selectedConnections.length} connections to clipboard`
                }
            } catch (error) {
                return {
                    content: `❌ Failed to copy to clipboard`
                }
            }
        }
    },

    // === UTILITY COMMANDS ===
    {
        id: 'help',
        name: 'Help',
        icon: '❓',
        description: 'Show available commands',
        parameters: [],
        execute: async () => {
            const commands = designEditorCommands
            const commandsList = commands.map(cmd => {
                const hotkey = cmd.hotkey ? ` (${cmd.hotkey})` : ''
                return `${cmd.icon || '⚡'} ${cmd.name}${hotkey}
  - ${cmd.description}`
            }).join('\n\n')

            return {
                content: `📚 Available Commands:

${commandsList}`
            }
        }
    },

    {
        id: 'clear',
        name: 'Clear',
        icon: '🧹',
        description: 'Clear the console',
        parameters: [],
        editorOnly: true,
        execute: async () => ({ content: null })
    }
]

// Register all design editor commands
designEditorCommands.forEach(command => {
    commandRegistry.register(command)
})
