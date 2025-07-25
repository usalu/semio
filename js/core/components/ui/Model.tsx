// #region Header

// Model.tsx

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
// #region Header

// Model.tsx

// 2025 Ueli Saluz
// 2025 AdrianoCelentano

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// #endregion

import {
  Environment,
  GizmoHelper,
  GizmoViewport,
  Grid,
  Line,
  OrbitControls,
  Select,
  TransformControls,
  useGLTF
} from '@react-three/drei'
import { Canvas, ThreeEvent } from '@react-three/fiber'
import {
  DiffStatus,
  Piece,
  Plane,
  ToSemioRotation,
  Vector,
  applyDesignDiff,
  findDesign,
  flattenDesign,
  getPieceRepresentationUrls,
  planeToMatrix,
  sameDesign
} from '@semio/js'
import React, { FC, JSX, useCallback, useEffect, useMemo, useRef, useState } from 'react'
import * as THREE from 'three'
import { DesignEditorAction, DesignEditorDispatcher, DesignEditorState } from './DesignEditor'

const getComputedColor = (variable: string): string => {
  return getComputedStyle(document.documentElement).getPropertyValue(variable).trim()
}

interface PlaneThreeProps {
  plane: Plane
}

const PlaneThree: FC<PlaneThreeProps> = ({ plane }) => {
  const matrix = useMemo(() => {
    return planeToMatrix(plane)
  }, [plane])
  return (
    <group matrix={matrix} matrixAutoUpdate={false}>
      <Line
        points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(1, 0, 0)]}
        color={new THREE.Color(getComputedColor('--color-primary'))}
      />
      <Line
        points={[new THREE.Vector3(0, 0, 0), new THREE.Vector3(0, 1, 0)]}
        color={new THREE.Color(getComputedColor('--color-primary'))}
      />
    </group>
  )
}

interface ModelPieceProps {
  piece: Piece
  plane: Plane
  fileUrl: string
  selected?: boolean
  updating?: boolean
  diffStatus?: DiffStatus
  onSelect: (piece: Piece) => void
  onPieceUpdate: (piece: Piece) => void
}

const ModelPiece: FC<ModelPieceProps> = ({
  piece,
  plane,
  fileUrl,
  selected,
  updating,
  diffStatus = DiffStatus.Unchanged,
  onSelect,
  onPieceUpdate
}) => {
  const [drag, setDrag] = useState<Vector | null>(null)
  const [dragKey, setDragKey] = useState(0)
  const fixed = piece.plane !== undefined
  const matrix = useMemo(() => {
    // const draggedPlane = drag ? { ...plane, origin: { x: plane.origin.x + drag?.x, y: plane.origin.y + drag?.y, z: plane.origin.z + drag?.z } } : plane
    const planeRotationMatrix = planeToMatrix(plane)
    planeRotationMatrix.multiply(ToSemioRotation())
    return planeRotationMatrix
  }, [plane, drag])
  const baseScene = useMemo(() => useGLTF(fileUrl).scene.clone(), [fileUrl])

  const getMaterial = (color: string, opacity = 1) =>
    new THREE.MeshBasicMaterial({ color, transparent: opacity < 1, opacity })

  const applyMaterial = (scene: THREE.Group, color: string, opacity = 1) => {
    scene.traverse((object) => {
      if (object instanceof THREE.Mesh) {
        object.material = getMaterial(color, opacity)
      } else if (object instanceof THREE.Line) {
        object.material = new THREE.LineBasicMaterial({ color, transparent: opacity < 1, opacity })
      }
    })
    return scene
  }

  const styledScene = useMemo(() => {
    if (diffStatus === DiffStatus.Added) return applyMaterial(baseScene.clone(), 'green')
    if (diffStatus === DiffStatus.Removed) return applyMaterial(baseScene.clone(), 'red', 0.2)
    if (diffStatus === DiffStatus.Modified) return applyMaterial(baseScene.clone(), 'yellow')
    if (selected) return applyMaterial(baseScene.clone(), getComputedColor('--color-primary'))
    if (updating) return applyMaterial(baseScene.clone(), getComputedColor('--color-foreground'), 0.1)
    return baseScene.clone()
  }, [baseScene, diffStatus, selected, updating])

  // Update selectedScene and updatingScene to use baseScene
  const selectedScene = useMemo(() => {
    const sceneClone = baseScene.clone()
    sceneClone.traverse((object) => {
      if (object instanceof THREE.Mesh) {
        const meshColor = new THREE.Color(getComputedColor('--color-primary'))
        object.material = new THREE.MeshBasicMaterial({ color: meshColor })
      }
      if (object instanceof THREE.Line) {
        const lineColor = new THREE.Color(getComputedColor('--color-foreground'))
        object.material = new THREE.LineBasicMaterial({ color: lineColor })
      }
    })
    return sceneClone
  }, [baseScene])

  const updatingScene = useMemo(() => {
    const sceneClone = baseScene.clone()
    sceneClone.traverse((object) => {
      if (object instanceof THREE.Mesh) {
        const meshColor = new THREE.Color(getComputedColor('--color-foreground'))
        object.material = new THREE.MeshBasicMaterial({ color: meshColor, transparent: true, opacity: 0.1 })
      }
      if (object instanceof THREE.Line) {
        const lineColor = new THREE.Color(getComputedColor('--color-background'))
        object.material = new THREE.LineBasicMaterial({ color: lineColor, transparent: true, opacity: 0.15 })
      }
    })
    return sceneClone
  }, [baseScene])

  const handleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      onSelect(piece)
      e.stopPropagation()
    },
    [onSelect, piece]
  )

  const transformControlRef = useRef(null)
  const handleMouseUp = useCallback(
    (e?: THREE.Event) => {
      console.log('handleMouseUp', e)
    },
    [plane]
  )

  const transformControl = selected && fixed
  if (transformControl) {
    console.log('transformControl', transformControl)
  }
  const group = (
    <group
      matrix={matrix}
      matrixAutoUpdate={false}
      // position={[plane!.origin.x, plane!.origin.z, -plane!.origin.y]}
      userData={{ pieceId: piece.id_ }}
      onClick={handleClick}
    >
      <primitive object={styledScene} />
    </group>
  )

  return transformControl ? (
    <TransformControls ref={transformControlRef} enabled={selected && fixed} onMouseUp={handleMouseUp}>
      {group}
    </TransformControls>
  ) : (
    group
  )
}

interface ModelDesignProps {
  designEditorState: DesignEditorState
  designEditorDispatcher: DesignEditorDispatcher
}

const ModelDesign: FC<ModelDesignProps> = ({ designEditorState, designEditorDispatcher }) => {
  const { kit, designId, fileUrls, selection, designDiff } = designEditorState
  const design = findDesign(kit, designId)
  if (!design) {
    return null
  }
  const types = kit?.types ?? []
  const piecePlanes = useMemo(() => {
    const flatDesign = flattenDesign(kit, designId)
    return flatDesign.pieces?.map((p) => p.plane!) || []
  }, [kit, designId])

  // Use inplace mode to get all pieces including removed ones with diff qualities
  const effectiveDesign = useMemo(() => applyDesignDiff(design, designDiff, true), [design, designDiff])
  const effectiveKit = { ...kit, designs: kit.designs?.map((d) => (sameDesign(design, d) ? effectiveDesign : d)) ?? [] }
  const piecePlanesFromEffectiveDesign = useMemo(() => {
    const flatDesign = flattenDesign(effectiveKit, designId)
    return flatDesign.pieces?.map((p) => p.plane!) || []
  }, [effectiveKit, designId])

  const pieceRepresentationUrls = useMemo(() => {
    return getPieceRepresentationUrls(effectiveDesign, types)
  }, [effectiveDesign, types])

  useEffect(() => {
    fileUrls.forEach((url, id) => {
      useGLTF.preload(id)
    })
  }, [fileUrls])

  effectiveDesign.pieces?.forEach((p) => {
    const type = types.find((t) => t.name === p.type.name && (t.variant || '') === (p.type.variant || ''))
    if (!type) throw new Error(`Type (${p.type.name}, ${p.type.variant}) for piece ${p.id_} not found`)
  })

  useEffect(() => {
    pieceRepresentationUrls.forEach((url, id) => {
      if (!fileUrls.has(url)) throw new Error(`Representation url ${url} for piece ${id} not found in fileUrls map`)
    })
  }, [pieceRepresentationUrls, fileUrls])

  function getPieceDiffFromQuality(piece: Piece): DiffStatus {
    const diffQuality = piece.qualities?.find((q) => q.name === 'semio.diffStatus')
    return (diffQuality?.value as DiffStatus) || DiffStatus.Unchanged
  }

  const pieceDiffStatuses = useMemo(() => {
    return effectiveDesign.pieces?.map((piece) => getPieceDiffFromQuality(piece)) || []
  }, [effectiveDesign])

  const handleSelectionChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId)

      if (
        !Array.isArray(selection.selectedPieceIds) ||
        newSelectedPieceIds.length !== selection.selectedPieceIds.length ||
        newSelectedPieceIds.some((id, index) => id !== selection.selectedPieceIds[index])
      ) {
        designEditorDispatcher({
          type: DesignEditorAction.SetSelection,
          payload: {
            ...selection,
            selectedPieceIds: newSelectedPieceIds
          }
        })
      }
    },
    [selection, designEditorDispatcher]
  )

  const handlePieceSelect = useCallback(
    (piece: Piece) => {
      if (selection.selectedPieceIds.includes(piece.id_)) {
        designEditorDispatcher({
          type: DesignEditorAction.SetSelection,
          payload: {
            ...selection,
            selectedPieceIds: selection.selectedPieceIds.filter((id) => id !== piece.id_)
          }
        })
      } else {
        designEditorDispatcher({
          type: DesignEditorAction.SetSelection,
          payload: {
            ...selection,
            selectedPieceIds: [...selection.selectedPieceIds, piece.id_]
          }
        })
      }
    },
    [selection, designEditorDispatcher]
  )

  const onPieceUpdate = useCallback(
    (piece: Piece) => {
      if (!design) return

      const newDesign = { ...design, pieces: design.pieces?.map((p) => p.id_ === piece.id_ ? piece : p) }

      designEditorDispatcher({
        type: DesignEditorAction.SetDesign,
        payload: newDesign
      })
    },
    [design, designEditorDispatcher]
  )
  return (
    <Select box multiple onChange={handleSelectionChange} filter={(items) => items}>
      <group quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}>
        {effectiveDesign.pieces?.map((piece, index) => (
          <ModelPiece
            key={`piece-${piece.id_}`}
            piece={piece}
            plane={piecePlanesFromEffectiveDesign[index!]}
            fileUrl={fileUrls.get(pieceRepresentationUrls.get(piece.id_)!)!}
            selected={selection.selectedPieceIds.includes(piece.id_)}
            diffStatus={pieceDiffStatuses[index]}
            onSelect={handlePieceSelect}
            onPieceUpdate={onPieceUpdate}
          />
          // <PlaneThree key={`plane-${piece.id_}`} plane={piecePlanes![index]} />
        ))}
      </group>
    </Select>
  )
}

const Gizmo: FC = (): JSX.Element => {
  const colors = useMemo(
    () =>
      [
        getComputedColor('--color-primary'),
        getComputedColor('--color-tertiary'),
        getComputedColor('--color-secondary')
      ] as [string, string, string],
    []
  )

  return (
    <GizmoHelper alignment="bottom-right" margin={[80, 80]}>
      <GizmoViewport
        labels={['X', 'Z', '-Y']}
        axisColors={colors}

      // font='Anta'
      />
    </GizmoHelper>
  )
}

interface ModelProps {
  designEditorState: DesignEditorState
  designEditorDispatcher: DesignEditorDispatcher
}
const Model: FC<ModelProps> = ({ designEditorState, designEditorDispatcher }) => {
  const [gridColors, setGridColors] = useState({
    sectionColor: getComputedColor('--foreground'),
    cellColor: getComputedColor('--accent-foreground')
  })
  useEffect(() => {
    const updateColors = () => {
      setGridColors({
        sectionColor: getComputedColor('--foreground'),
        cellColor: getComputedColor('--accent-foreground')
      })
    }
    updateColors()
    const observer = new MutationObserver(updateColors)
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['class']
    })

    return () => observer.disconnect()
  }, [])

  const handleDoubleClick = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation()
      designEditorDispatcher({
        type: DesignEditorAction.SetFullscreen,
        payload: designEditorState.fullscreenPanel === 'model' ? null : 'model'
      })
    },
    [designEditorState.fullscreenPanel, designEditorDispatcher]
  )

  const handlePointerMissed = useCallback(() => {
    designEditorDispatcher({
      type: DesignEditorAction.SetSelection,
      payload: {
        selectedPieceIds: [],
        selectedConnections: []
      }
    })
  }, [designEditorDispatcher])

  const fullscreen = designEditorState.fullscreenPanel === 'model'

  return (
    <div id="model" className="w-full h-full">
      <Canvas onDoubleClickCapture={handleDoubleClick} onPointerMissed={handlePointerMissed}>
        <OrbitControls
          makeDefault
          mouseButtons={{
            LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
            MIDDLE: undefined,
            RIGHT: undefined // Right button disabled to allow selection
          }}
        />
        <ambientLight intensity={1} />
        {/* <Suspense fallback={null}>
                        <Gltf src={src} />
                    </Suspense> */}
        <ModelDesign designEditorState={designEditorState} designEditorDispatcher={designEditorDispatcher} />
        <Environment files={'schlenker-shed.hdr'} />
        <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />
        {fullscreen && <Gizmo />}
      </Canvas>
    </div>
  )
}

export default Model
