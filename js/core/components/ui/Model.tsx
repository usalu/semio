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
  Piece,
  Plane,
  Status,
  ToSemioRotation,
  applyDesignDiff,
  flattenDesign,
  getDesign,
  getPieceRepresentationUrls,
  planeToMatrix
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
  status?: Status
  onSelect: (piece: Piece) => void
}

const ModelPiece: FC<ModelPieceProps> = ({
  piece,
  plane,
  fileUrl,
  selected,
  updating,
  status = Status.Unchanged,
  onSelect
}) => {
  const fixed = piece.plane !== undefined
  const matrix = useMemo(() => {
    // const threeToSemioRotation = new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
    // const semioToThreeRotation = new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
    // // const origin = new THREE.Vector3(plane.origin.x, plane.origin.y, plane.origin.z);
    // const xAxis = new THREE.Vector3(plane.xAxis.x, plane.xAxis.y, plane.xAxis.z);
    // const yAxis = new THREE.Vector3(plane.yAxis.x, plane.yAxis.y, plane.yAxis.z);
    // const zAxis = new THREE.Vector3().crossVectors(xAxis, yAxis);
    // const planeRotationMatrix = new THREE.Matrix4();
    // planeRotationMatrix.makeBasis(xAxis.normalize(), yAxis.normalize(), zAxis.normalize());
    // // planeRotationMatrix.setPosition(origin);
    // const m = new THREE.Matrix4();
    // m.multiply(threeToSemioRotation);
    // m.multiply(planeRotationMatrix);
    // m.multiply(semioToThreeRotation);
    // m.multiply(new THREE.Matrix4().makeTranslation(plane.origin.x, -plane.origin.z, plane.origin.y));
    // return m
    const planeRotationMatrix = planeToMatrix(plane)
    planeRotationMatrix.multiply(ToSemioRotation())
    return planeRotationMatrix
  }, [plane])
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
    if (status === Status.Added) return applyMaterial(baseScene.clone(), 'green')
    if (status === Status.Removed) return applyMaterial(baseScene.clone(), 'red', 0.2)
    if (status === Status.Modified) return applyMaterial(baseScene.clone(), 'yellow')
    if (selected) return applyMaterial(baseScene.clone(), getComputedColor('--color-primary'))
    if (updating) return applyMaterial(baseScene.clone(), getComputedColor('--color-foreground'), 0.1)
    return baseScene.clone()
  }, [baseScene, status, selected, updating])

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
  const design = getDesign(kit, designId)
  if (!design) {
    return null
  }
  const types = kit?.types ?? []
  const piecePlanes = useMemo(() => {
    const flatDesign = flattenDesign(kit, designId)
    return flatDesign.pieces?.map((p) => p.plane!) || []
  }, [kit, designId])

  // Use inplace mode to get all pieces including removed ones with status qualities
  const effectiveDesign = useMemo(() => applyDesignDiff(design, designDiff, true), [design, designDiff])

  const tempKit = {
    ...kit,
    designs:
      kit.designs?.map((d) => {
        if (getDesign(kit, designId) === d) {
          return effectiveDesign
        }
        return d
      }) ?? []
  }
  const piecePlanesFromEffectiveDesign = useMemo(() => {
    const flatDesign = flattenDesign(tempKit, designId)
    return flatDesign.pieces?.map((p) => p.plane!) || []
  }, [tempKit, designId])

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

  function getPieceStatusFromQuality(piece: Piece): Status {
    const statusQuality = piece.qualities?.find((q) => q.name === 'semio.status')
    return (statusQuality?.value as Status) || Status.Unchanged
  }

  const pieceStatuses = useMemo(() => {
    return effectiveDesign.pieces?.map((piece) => getPieceStatusFromQuality(piece)) || []
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
            status={pieceStatuses[index]}
            onSelect={handlePieceSelect}
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
