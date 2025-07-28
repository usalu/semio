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
  OrthographicCamera,
  Select,
  TransformControls,
  useGLTF
} from '@react-three/drei'
import { Canvas, ThreeEvent } from '@react-three/fiber'
import {
  Design,
  DesignEditorAction,
  DesignEditorDispatcher,
  DesignEditorState,
  DiffStatus,
  FullscreenPanel,
  Piece,
  Plane,
  ToSemioRotation,
  applyDesignDiff,
  findDesign,
  flattenDesign,
  getPieceRepresentationUrls,
  planeToMatrix,
  sameDesign
} from '@semio/js'
import React, { FC, JSX, useCallback, useEffect, useMemo, useRef, useState } from 'react'
import * as THREE from 'three'

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
  onSelect: (piece: Piece, e?: MouseEvent) => void
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
  const fixed = piece.plane !== undefined
  const matrix = useMemo(() => {
    const planeRotationMatrix = planeToMatrix(plane)
    planeRotationMatrix.multiply(ToSemioRotation())
    return planeRotationMatrix
  }, [plane])
  const styledScene = useMemo(() => {
    const scene = useGLTF(fileUrl).scene.clone()
    let meshColor: THREE.Color
    let meshOpacity = 1
    let lineOpacity = 1

    if (diffStatus === DiffStatus.Added) {
      meshColor = new THREE.Color(getComputedColor('--color-success'))
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor('--color-primary'))
        meshColor.lerp(selectedColor, 0.5)
      }
    } else if (diffStatus === DiffStatus.Modified) {
      meshColor = new THREE.Color(getComputedColor('--color-warning'))
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor('--color-primary'))
        meshColor.lerp(selectedColor, 0.5)
      }
    } else if (diffStatus === DiffStatus.Removed) {
      meshColor = new THREE.Color(getComputedColor('--color-error'))
      meshOpacity = 0.2
      lineOpacity = 0.25
      if (selected) {
        const selectedColor = new THREE.Color(getComputedColor('--color-primary'))
        meshColor.lerp(selectedColor, 0.5)
      }
    } else if (selected) {
      meshColor = new THREE.Color(getComputedColor('--color-primary'))
    } else {
      return scene
    }

    const lineColor = new THREE.Color(getComputedColor('--color-foreground'))
    scene.traverse((object) => {
      if (object instanceof THREE.Mesh) {
        object.material = new THREE.MeshBasicMaterial({
          color: meshColor,
          transparent: meshOpacity < 1,
          opacity: meshOpacity
        })
      }
      if (object instanceof THREE.Line) {
        object.material = new THREE.LineBasicMaterial({
          color: lineColor,
          transparent: lineOpacity < 1,
          opacity: lineOpacity
        })
      }
    })
    return scene
  }, [fileUrl, diffStatus, selected])

  const handleClick = useCallback(
    (e: ThreeEvent<MouseEvent>) => {
      onSelect(piece, e.nativeEvent)
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

  //#region Actions
  const onDesignChange = useCallback((d: Design) => designEditorDispatcher({ type: DesignEditorAction.SetDesign, payload: d }), [designEditorDispatcher])
  const onSelectPiece = useCallback((p: Piece) => designEditorDispatcher({ type: DesignEditorAction.SelectPiece, payload: p }), [designEditorDispatcher])
  const onAddPieceToSelection = useCallback((p: Piece) => designEditorDispatcher({ type: DesignEditorAction.AddPieceToSelection, payload: p }), [designEditorDispatcher])
  const onRemovePieceFromSelection = useCallback((p: Piece) => designEditorDispatcher({ type: DesignEditorAction.RemovePieceFromSelection, payload: p }), [designEditorDispatcher])
  const onSetPieceInDesign = useCallback((p: Piece) => designEditorDispatcher({ type: DesignEditorAction.SetPieceInDesign, payload: p }), [designEditorDispatcher])
  const onSelectPieces = useCallback((pieceIds: string[]) => designEditorDispatcher({ type: DesignEditorAction.SelectPieces, payload: pieceIds.map(id => ({ id_: id })) }), [designEditorDispatcher])
  //#endregion Actions

  const onChange = useCallback(
    (selected: THREE.Object3D[]) => {
      const newSelectedPieceIds = selected.map((item) => item.parent?.userData.pieceId).filter(Boolean)

      if (
        !Array.isArray(selection.selectedPieceIds) ||
        newSelectedPieceIds.length !== selection.selectedPieceIds.length ||
        newSelectedPieceIds.some((id, index) => id !== selection.selectedPieceIds[index])
      ) {
        onSelectPieces(newSelectedPieceIds)
      }
    },
    [selection, onSelectPieces]
  )

  const onSelect = useCallback(
    (piece: Piece, e?: MouseEvent) => {
      if (e?.ctrlKey || e?.metaKey) {
        onRemovePieceFromSelection(piece)
      } else if (e?.shiftKey) {
        onAddPieceToSelection(piece)
      } else {
        onSelectPiece(piece)
      }
    },
    [onRemovePieceFromSelection, onAddPieceToSelection, onSelectPiece]
  )

  const onPieceUpdate = useCallback(
    (piece: Piece) => {
      if (!design) return
      onSetPieceInDesign(piece)
    },
    [design, onSetPieceInDesign]
  )
  return (
    <Select box multiple onChange={onChange} filter={(items) => items}>
      <group quaternion={new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476)}>
        {effectiveDesign.pieces?.map((piece, index) => (
          <ModelPiece
            key={`piece-${piece.id_}`}
            piece={piece}
            plane={piecePlanesFromEffectiveDesign[index!]}
            fileUrl={fileUrls.get(pieceRepresentationUrls.get(piece.id_)!)!}
            selected={selection.selectedPieceIds.includes(piece.id_)}
            diffStatus={pieceDiffStatuses[index]}
            onSelect={onSelect}
            onPieceUpdate={onPieceUpdate}
          />
          // <PlaneThree key={`plane-${piece.id_}`} plane={piecePlanes![index]} />
        ))}
      </group>
    </Select>
  )
}

const Gizmo: FC = (): JSX.Element => {
  const colors = useMemo(() => [getComputedColor('--color-primary'), getComputedColor('--color-tertiary'), getComputedColor('--color-secondary')] as [string, string, string], [])
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

const ModelCore: FC<ModelProps> = ({ designEditorState, designEditorDispatcher }) => {
  const fullscreen = designEditorState.fullscreenPanel === FullscreenPanel.Model
  const [gridColors, setGridColors] = useState({
    sectionColor: getComputedColor('--foreground'),
    cellColor: getComputedColor('--accent-foreground')
  })
  useEffect(() => {
    const updateColors = () => { setGridColors({ sectionColor: getComputedColor('--foreground'), cellColor: getComputedColor('--accent-foreground') }) }
    updateColors()
    const observer = new MutationObserver(updateColors)
    observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
    return () => observer.disconnect()
  }, [])
  const camera = useRef<THREE.OrthographicCamera>(null)
  return (
    <>
      <OrthographicCamera ref={camera} />
      <OrbitControls
        makeDefault
        mouseButtons={{
          LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
          MIDDLE: undefined,
          RIGHT: undefined // Right button disabled to allow selection
        }}
      />
      <ambientLight intensity={1} />
      {/* <Stage center={{ disable: true }} environment={null}> */}
      <ModelDesign designEditorState={designEditorState} designEditorDispatcher={designEditorDispatcher} />
      {/* </Stage> */}
      <Environment files={'schlenker-shed.hdr'} />
      <Grid infiniteGrid={true} sectionColor={gridColors.sectionColor} cellColor={gridColors.cellColor} />
      {fullscreen && <Gizmo />}
    </>
  )
}

interface ModelProps {
  designEditorState: DesignEditorState
  designEditorDispatcher: DesignEditorDispatcher
}
const Model: FC<ModelProps> = ({ designEditorState, designEditorDispatcher }) => {
  //#region Actions  
  const onDeselectAll = useCallback(() => designEditorDispatcher({ type: DesignEditorAction.DeselectAll, payload: null }), [designEditorDispatcher])
  const onToggleModelFullscreen = useCallback(() => designEditorDispatcher({ type: DesignEditorAction.ToggleModelFullscreen, payload: null }), [designEditorDispatcher])
  //#endregion Actions

  const onDoubleClickCapture = useCallback((e: React.MouseEvent) => { e.stopPropagation(); onToggleModelFullscreen() }, [onToggleModelFullscreen])
  const onPointerMissed = useCallback((e: MouseEvent) => { if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) onDeselectAll() }, [onDeselectAll])
  return (
    <div id="model" className="h-full w-full">
      <Canvas onDoubleClickCapture={onDoubleClickCapture} onPointerMissed={onPointerMissed}>
        <ModelCore designEditorState={designEditorState} designEditorDispatcher={designEditorDispatcher} />
      </Canvas>
    </div>
  )
}

export default Model
