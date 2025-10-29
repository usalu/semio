// #region Header

// Details.tsx

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

import { arrayMove } from "@dnd-kit/sortable";
import { Minus, Plus } from "lucide-react";
import { FC, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { SortableTreeItems, TreeContent, TreeItem } from "../../../../elements/aggregation/Tree";
import Combobox from "../../../../elements/input/Combobox";
import { Input } from "../../../../elements/input/Input";
import { Slider } from "../../../../elements/input/Slider";
import Stepper from "../../../../elements/input/Stepper";
import { Textarea } from "../../../../elements/input/Textarea";
import { Connection, Design, Guid, Kit, Piece, findDesignInKit, findPieceInDesign, findTypeInKit, guid } from "../../../../semio";
import { useDesign, useIsInDesignScope, useKit, useKitCommands, usePiecesFromIds, useReplacableDesigns, useReplacableTypes, useTooltip } from "../../../store";
import { useDesignAppCommands, useDesignAppSelection } from "../store";

export const DesignSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <DesignSectionForm />;
};

const DesignSectionForm: FC = () => {
  const { t } = useTranslation();
  const tooltip = useTooltip();
  const { startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  const kit = useKit();
  const kitCommands = useKitCommands();
  const design = useDesign() as Design;

  const updateDesignField = (diff: any) => {
    kitCommands.updateDesign(design.guid, diff);
  };

  const handleChange = (updatedDesign: any) => {
    kitCommands.updateDesign(design.guid, updatedDesign);
  };

  const addLocation = () => {
    startTransaction();
    updateDesignField({ location: { guid: guid(), longitude: 0, latitude: 0 } });
    finalizeTransaction();
  };

  const removeLocation = () => {
    startTransaction();
    updateDesignField({ location: undefined });
    finalizeTransaction();
  };

  return (
    <>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.name" value={design.name} onLazyChange={(value) => updateDesignField({ name: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Textarea lazy i18n="semio.design.panel.details.description" value={design.description || ""} placeholder={t("semio.design.descriptionPlaceholder")} onLazyChange={(value) => updateDesignField({ description: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.icon" value={design.icon || ""} placeholder={t("semio.design.iconPlaceholder")} onLazyChange={(value) => updateDesignField({ icon: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.image" value={design.image || ""} placeholder={t("semio.design.imagePlaceholder")} onLazyChange={(value) => updateDesignField({ image: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.variant" value={design.variant || ""} placeholder={t("semio.design.variantPlaceholder")} onLazyChange={(value) => updateDesignField({ variant: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.view" value={design.view || ""} placeholder={t("semio.design.viewPlaceholder")} onLazyChange={(value) => updateDesignField({ view: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Input lazy i18n="semio.design.panel.details.unit" value={design.unit || ""} onLazyChange={(value) => updateDesignField({ unit: value })} startTransaction={startTransaction} finalizeTransaction={finalizeTransaction} abortTransaction={abortTransaction} />
        </TreeContent>
      </TreeItem>
      {design.location ? (
        <TreeItem
          label={t("semio.design.location")}
          actions={[
            {
              icon: <Minus />,
              onClick: removeLocation,
              title: t("semio.common.remove"),
            },
          ]}
        >
          <TreeItem>
            <TreeContent>
              <Stepper
                i18n="semio.design.panel.details.location.longitude"
                value={design.location.longitude}
                onChange={(value) =>
                  handleChange({
                    ...design,
                    location: { ...design.location!, longitude: value },
                  })
                }
                startTransaction={startTransaction}
                finalizeTransaction={finalizeTransaction}
                abortTransaction={abortTransaction}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                i18n="semio.design.panel.details.location.latitude"
                value={design.location.latitude}
                onChange={(value) =>
                  handleChange({
                    ...design,
                    location: { ...design.location!, latitude: value },
                  })
                }
                startTransaction={startTransaction}
                finalizeTransaction={finalizeTransaction}
                abortTransaction={abortTransaction}
                step={0.000001}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      ) : (
        <TreeItem
          label={t("semio.design.location")}
          actions={[
            {
              icon: <Plus />,
              onClick: addLocation,
              title: t("semio.common.add"),
            },
          ]}
        />
      )}
      <TreeItem
        label={t("semio.design.authors")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              handleChange({
                ...design,
                authors: [...(design.authors || []), { name: "", email: "" }],
              });
              finalizeTransaction();
            },
            title: t("semio.common.add"),
          },
        ]}
      >
        {design.authors && design.authors.length > 0 && (
          <SortableTreeItems
            items={(design.authors || []).map((author: any, index: number) => ({
              ...author,
              id: `author-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction();
              handleChange({
                ...design,
                authors: arrayMove(design.authors!, oldIndex, newIndex),
              });
              finalizeTransaction();
            }}
          >
            {(author, index) => (
              <TreeItem
                key={`author-${index}`}
                label={author.name || `${t("semio.design.author")} ${index + 1}`}
                sortable={true}
                sortableId={`author-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      startTransaction();
                      handleChange({
                        ...design,
                        authors: design.authors?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction();
                    },
                    title: t("semio.common.remove"),
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.author.name"
                      value={author.name}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          name: e.target.value,
                        };
                        handleChange({ ...design, authors: updatedAuthors });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.author.email"
                      value={author.email}
                      onChange={(e) => {
                        const updatedAuthors = [...(design.authors || [])];
                        updatedAuthors[index] = {
                          ...author,
                          email: e.target.value,
                        };
                        handleChange({ ...design, authors: updatedAuthors });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      <TreeItem
        label={t("semio.design.attributes")}
        actions={[
          {
            icon: <Plus />,
            onClick: () => {
              startTransaction();
              updateDesignField({
                attributes: [...(design.attributes || []), { key: "" }],
              });
              finalizeTransaction();
            },
            title: t("semio.common.add"),
          },
        ]}
      >
        {design.attributes && design.attributes.length > 0 && (
          <SortableTreeItems
            items={(design.attributes || []).map((attribute: any, index: number) => ({
              ...attribute,
              id: `attribute-${index}`,
              index,
            }))}
            onReorder={(oldIndex, newIndex) => {
              startTransaction();
              updateDesignField({
                attributes: arrayMove(design.attributes!, oldIndex, newIndex),
              });
              finalizeTransaction();
            }}
          >
            {(attribute, index) => (
              <TreeItem
                key={`attribute-${index}`}
                label={attribute.key || `${t("semio.design.attribute")} ${index + 1}`}
                sortable={true}
                sortableId={`attribute-${index}`}
                isDragHandle={true}
                actions={[
                  {
                    icon: <Minus />,
                    onClick: () => {
                      startTransaction();
                      updateDesignField({
                        attributes: design.attributes?.filter((_: any, i: number) => i !== index),
                      });
                      finalizeTransaction();
                    },
                    title: t("semio.common.remove"),
                  },
                ]}
              >
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.attribute.name"
                      value={attribute.key}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          key: e.target.value,
                        };
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.attribute.value"
                      value={attribute.value || ""}
                      placeholder={t("semio.design.attributeValuePlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          value: e.target.value,
                        };
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.attribute.unit"
                      value={attribute.unit || ""}
                      placeholder={t("semio.design.attributeUnitPlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          unit: e.target.value,
                        };
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
                <TreeItem>
                  <TreeContent>
                    <Input
                      i18n="semio.design.panel.details.attribute.definition"
                      value={attribute.definition || ""}
                      placeholder={t("semio.design.attributeDefinitionPlaceholder")}
                      onChange={(e) => {
                        const updatedAttributes = [...(design.attributes || [])];
                        updatedAttributes[index] = {
                          ...attribute,
                          definition: e.target.value,
                        };
                        updateDesignField({ attributes: updatedAttributes });
                      }}
                      onFocus={startTransaction}
                      onBlur={finalizeTransaction}
                    />
                  </TreeContent>
                </TreeItem>
              </TreeItem>
            )}
          </SortableTreeItems>
        )}
      </TreeItem>
      {design.createdAt && (
        <TreeItem>
          <TreeContent>
            <Input
              i18n="semio.design.panel.details.createdAt"
              value={(() => {
                const date = design.createdAt;
                if (date instanceof Date) return date.toISOString().split("T")[0];
                if (typeof date === "string") return (date as string).split("T")[0];
                return "";
              })()}
              disabled
            />
          </TreeContent>
        </TreeItem>
      )}
      {design.updatedAt && (
        <TreeItem>
          <TreeContent>
            <Input
              i18n="semio.design.panel.details.updatedAt"
              value={(() => {
                const date = design.updatedAt;
                if (date instanceof Date) return date.toISOString().split("T")[0];
                if (typeof date === "string") return (date as string).split("T")[0];
                return "";
              })()}
              disabled
            />
          </TreeContent>
        </TreeItem>
      )}
    </>
  );
};

export const PiecesSection: FC = () => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PiecesSectionForm />;
};

const PiecesSectionForm: FC = () => {
  const { t } = useTranslation();
  const { startTransaction, finalizeTransaction, abortTransaction, updatePiece, updatePieces } = useDesignAppCommands();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;
  // const metadata = usePiecesMetadata();
  const metadata = new Map();
  const selection = useDesignAppSelection();
  const pieces = usePiecesFromIds(selection.pieces || []);
  // const includedDesigns = useIncludedDesigns();

  // const includedDesignMap = useMemo(() => new Map(includedDesigns.map((d) => [d.id, d])), [includedDesigns]);

  const isSingle = pieces.length === 1;
  const piece = isSingle ? pieces[0] : null;

  // Check if we're dealing with design pieces
  const isDesignPiece = isSingle ? typeof piece?.type === "string" && piece?.type === "design" : pieces.every((p) => typeof p.type === "string" && p.type === "design");
  const hasDesignPieces = pieces.some((p) => typeof p.type === "string" && p.type === "design");
  const hasMixedTypes = hasDesignPieces && pieces.some((p) => typeof p.type === "string" && p.type !== "design");

  const getCommonValue = <T,>(getter: (piece: Piece) => T | undefined): T | undefined => {
    const values = pieces.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  const getPieceId = (piece: Piece): string => {
    return piece.guid;
  };

  const handleTypeNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleTypeVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignNameChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignVariantChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const handleDesignViewChange = (value: string) => {
    // TODO: Implement using updatePiece/updatePieces commands
  };

  const fixPieces = async () => {
    // TODO: Implement using execute command
  };

  const handleCenterXChange = (value: number) => {
    if (isSingle && piece) {
      updatePiece(piece.guid, { center: { x: value, y: piece.center?.y ?? 0 } });
    } else {
      const updates = pieces.map((p) => ({ id: p.guid, diff: { center: { x: value, y: p.center?.y ?? 0 } } }));
      updatePieces(updates);
    }
  };

  const handleCenterYChange = (value: number) => {
    if (isSingle && piece) {
      updatePiece(piece.guid, { center: { x: piece.center?.x ?? 0, y: value } });
    } else {
      const updates = pieces.map((p) => ({ id: p.guid, diff: { center: { x: p.center?.x ?? 0, y: value } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneOriginXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, x: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneOriginYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, y: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneOriginZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, origin: { ...piece.plane.origin, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, origin: { ...p.plane!.origin, z: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneXAxisXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, x: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneXAxisYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, y: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneXAxisZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, xAxis: { ...piece.plane.xAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, xAxis: { ...p.plane!.xAxis, z: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneYAxisXChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, x: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, x: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneYAxisYChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, y: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, y: value } } } }));
      updatePieces(updates);
    }
  };

  const handlePlaneYAxisZChange = (value: number) => {
    if (isSingle && piece && piece.plane) {
      updatePiece(piece.guid, { plane: { ...piece.plane, yAxis: { ...piece.plane.yAxis, z: value } } });
    } else {
      const updates = pieces.filter((p) => p.plane).map((p) => ({ id: p.guid, diff: { plane: { ...p.plane!, yAxis: { ...p.plane!.yAxis, z: value } } } }));
      updatePieces(updates);
    }
  };

  const commonTypeName = getCommonValue((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.name;
  });
  const commonTypeVariant = getCommonValue((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.variant;
  });
  const commonCenterX = getCommonValue((p) => p.center?.x);
  const commonCenterY = getCommonValue((p) => p.center?.y);
  const commonPlaneOriginX = getCommonValue((p) => p.plane?.origin.x);
  const commonPlaneOriginY = getCommonValue((p) => p.plane?.origin.y);
  const commonPlaneOriginZ = getCommonValue((p) => p.plane?.origin.z);
  const commonPlaneXAxisX = getCommonValue((p) => p.plane?.xAxis.x);
  const commonPlaneXAxisY = getCommonValue((p) => p.plane?.xAxis.y);
  const commonPlaneXAxisZ = getCommonValue((p) => p.plane?.xAxis.z);
  const commonPlaneYAxisX = getCommonValue((p) => p.plane?.yAxis.x);
  const commonPlaneYAxisY = getCommonValue((p) => p.plane?.yAxis.y);
  const commonPlaneYAxisZ = getCommonValue((p) => p.plane?.yAxis.z);

  const hasCenter = pieces.some((p) => p.center);
  const hasPlane = pieces.some((p) => p.plane);
  const hasVariant = pieces.some((p) => {
    const type = p.type ? findTypeInKit(kit, p.type) : null;
    return type?.variant;
  });
  const hasUnfixedPieces = pieces.some((p) => !p.plane || !p.center);

  const pieceIds = useMemo(() => pieces.map((p) => p.guid), [pieces]);

  const selectedVariants = useMemo(
    () => [
      ...new Set(
        pieces
          .map((p) => {
            const type = p.type ? findTypeInKit(kit, p.type) : null;
            return type?.variant;
          })
          .filter((v): v is string => Boolean(v)),
      ),
    ],
    [pieces, kit],
  );
  const availableTypes = useReplacableTypes(pieceIds, isDesignPiece ? [] : selectedVariants);
  const availableTypeNames = useMemo(() => [...new Set(availableTypes.map((t) => t.name))], [availableTypes]);
  const allReplacableTypes = useReplacableTypes(pieceIds, []);
  const availableVariants = useMemo(
    () =>
      commonTypeName && !isDesignPiece
        ? [
            ...new Set(
              allReplacableTypes
                .filter((t) => t.name === commonTypeName)
                .map((t) => t.variant)
                .filter((v): v is string => Boolean(v)),
            ),
          ]
        : [],
    [commonTypeName, isDesignPiece, allReplacableTypes],
  );

  const replacableDesignsRaw = useReplacableDesigns(isSingle && piece ? piece : ({} as Piece));
  const availableDesigns = isDesignPiece && isSingle && piece ? replacableDesignsRaw : [];
  const availableDesignNames = useMemo(() => [...new Set(availableDesigns.map((d) => d.name))], [availableDesigns]);

  // Get piece type/design for pieces
  const pieceType = piece?.type && typeof piece.type === "string" && piece.type !== "design" ? findTypeInKit(kit, piece.type) : null;
  const pieceDesign = piece?.design && typeof piece.design === "string" ? findDesignInKit(kit, piece.design) : null;

  // Get available design variants and views
  const availableDesignVariants = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name)
            .map((d) => d.variant)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  const availableDesignViews = pieceDesign
    ? [
        ...new Set(
          availableDesigns
            .filter((d) => d.name === pieceDesign.name && (d.variant || "") === (pieceDesign.variant || ""))
            .map((d) => d.view)
            .filter((v): v is string => Boolean(v)),
        ),
      ]
    : [];

  let parentConnection: Connection | null = null;
  let parentConnections: Connection[] = [];

  // TODO: Re-implement parent connection finding once metadata is available
  // if (isSingle && piece) {
  //   const pieceMetadata = metadata.get(getPieceId(piece));
  //   if (pieceMetadata?.parentPieceId && pieceMetadata?.parentConnectionId) {
  //     try {
  //       parentConnection = findConnectionInDesign(design, pieceMetadata.parentConnectionId);
  //     } catch {}
  //   }
  // } else if (!isSingle) {
  //   // For multiple pieces, find all their parent connections
  //   parentConnections = pieces
  //     .map((piece) => {
  //       const pieceMetadata = metadata.get(getPieceId(piece));
  //       if (pieceMetadata?.parentPieceId && pieceMetadata?.parentConnectionId) {
  //         try {
  //           return findConnectionInDesign(design, pieceMetadata.parentConnectionId);
  //         } catch {
  //           return null;
  //         }
  //       }
  //       return null;
  //     })
  //     .filter((conn) => conn !== null) as Connection[];
  // }

  return (
    <>
      {hasMixedTypes ? (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">{t("semio.piece.mixedSelectionMessage")}</p>
          </TreeContent>
        </TreeItem>
      ) : (
        <>
          {isSingle && piece && (
            <TreeItem>
              <TreeContent>
                <Input label={t("semio.piece.id")} value={getPieceId(piece)} disabled />
              </TreeContent>
            </TreeItem>
          )}

          {isDesignPiece ? (
            // Design piece fields
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    label={t("semio.design.name")}
                    options={availableDesignNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={pieceDesign?.name || pieceType?.name || ""}
                    placeholder={t("semio.common.selectDesign")}
                    onValueChange={handleDesignNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {availableDesignVariants.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("semio.design.variant")}
                      options={availableDesignVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={pieceDesign?.variant || pieceType?.variant || ""}
                      placeholder={t("semio.common.selectVariant")}
                      onValueChange={handleDesignVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
              {availableDesignViews.length > 0 && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("semio.design.view")}
                      options={availableDesignViews.map((view) => ({
                        value: view,
                        label: view,
                      }))}
                      value={pieceDesign?.view || ""}
                      placeholder={t("semio.common.selectView")}
                      onValueChange={handleDesignViewChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          ) : (
            // Regular piece fields
            <>
              <TreeItem>
                <TreeContent>
                  <Combobox
                    label={t("semio.piece.type")}
                    options={availableTypeNames.map((name) => ({
                      value: name,
                      label: name,
                    }))}
                    value={isSingle && piece && piece.type ? findTypeInKit(kit, piece.type)?.name || "" : commonTypeName || ""}
                    placeholder={!isSingle && commonTypeName === undefined ? t("semio.common.mixedValues") : t("semio.common.selectType")}
                    onValueChange={handleTypeNameChange}
                  />
                </TreeContent>
              </TreeItem>
              {(hasVariant || availableVariants.length > 0) && (
                <TreeItem>
                  <TreeContent>
                    <Combobox
                      label={t("semio.type.variant")}
                      options={availableVariants.map((variant) => ({
                        value: variant,
                        label: variant,
                      }))}
                      value={isSingle && piece && piece.type ? findTypeInKit(kit, piece.type)?.variant || "" : commonTypeVariant || ""}
                      placeholder={!isSingle && commonTypeVariant === undefined ? t("semio.common.mixedValues") : t("semio.common.selectVariant")}
                      onValueChange={handleTypeVariantChange}
                      allowClear={true}
                    />
                  </TreeContent>
                </TreeItem>
              )}
            </>
          )}
        </>
      )}
      {hasCenter && (
        <TreeItem label={t("semio.piece.center")}>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("semio.common.x")}
                value={isSingle && piece ? piece.center?.x : commonCenterX}
                onChange={handleCenterXChange}
                startTransaction={startTransaction}
                finalizeTransaction={finalizeTransaction}
                abortTransaction={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Stepper
                label={t("semio.common.y")}
                value={isSingle && piece ? piece.center?.y : commonCenterY}
                onChange={handleCenterYChange}
                startTransaction={startTransaction}
                finalizeTransaction={finalizeTransaction}
                abortTransaction={abortTransaction}
                step={0.1}
              />
            </TreeContent>
          </TreeItem>
        </TreeItem>
      )}
      {hasPlane && (
        <TreeItem label={t("semio.piece.plane")}>
          <TreeItem label={t("semio.piece.planeOrigin")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.x")}
                  value={isSingle && piece ? piece.plane?.origin.x : commonPlaneOriginX}
                  onChange={handlePlaneOriginXChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.y")}
                  value={isSingle && piece ? piece.plane?.origin.y : commonPlaneOriginY}
                  onChange={handlePlaneOriginYChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.z")}
                  value={isSingle && piece ? piece.plane?.origin.z : commonPlaneOriginZ}
                  onChange={handlePlaneOriginZChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem label={t("semio.piece.planeXAxis")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.x")}
                  value={isSingle && piece ? piece.plane?.xAxis.x : commonPlaneXAxisX}
                  onChange={handlePlaneXAxisXChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.y")}
                  value={isSingle && piece ? piece.plane?.xAxis.y : commonPlaneXAxisY}
                  onChange={handlePlaneXAxisYChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.z")}
                  value={isSingle && piece ? piece.plane?.xAxis.z : commonPlaneXAxisZ}
                  onChange={handlePlaneXAxisZChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
          <TreeItem label={t("semio.piece.planeYAxis")}>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.x")}
                  value={isSingle && piece ? piece.plane?.yAxis.x : commonPlaneYAxisX}
                  onChange={handlePlaneYAxisXChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.y")}
                  value={isSingle && piece ? piece.plane?.yAxis.y : commonPlaneYAxisY}
                  onChange={handlePlaneYAxisYChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
            <TreeItem>
              <TreeContent>
                <Stepper
                  label={t("semio.common.z")}
                  value={isSingle && piece ? piece.plane?.yAxis.z : commonPlaneYAxisZ}
                  onChange={handlePlaneYAxisZChange}
                  startTransaction={startTransaction}
                  finalizeTransaction={finalizeTransaction}
                  abortTransaction={abortTransaction}
                  step={0.1}
                />
              </TreeContent>
            </TreeItem>
          </TreeItem>
        </TreeItem>
      )}
      {(parentConnection || parentConnections.length > 0) && (
        <div style={{ marginTop: "0.5rem" }}>
          <ConnectionsSection connections={isSingle && parentConnection ? [parentConnection] : parentConnections} sectionLabel={isSingle ? "Parent Connection" : `Parent Connections (${parentConnections.length})`} />
        </div>
      )}
    </>
  );
};

export const ConnectionsSection: FC<{
  connections: any[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <ConnectionsSectionForm connections={connections} sectionLabel={sectionLabel} />;
};

const ConnectionsSectionForm: FC<{
  connections: Connection[];
  sectionLabel?: string;
}> = ({ connections, sectionLabel }) => {
  const { t } = useTranslation();
  const { /* setConnection, setConnections, */ startTransaction, finalizeTransaction, abortTransaction } = useDesignAppCommands();
  const connectionObjects = connections;

  const isSingle = connections.length === 1;
  const connection = isSingle ? connectionObjects[0] : null;

  const getCommonValue = <T,>(getter: (connection: Connection) => T | undefined): T | undefined => {
    const values = connectionObjects.map(getter).filter((v) => v !== undefined);
    if (values.length === 0) return undefined;
    const firstValue = values[0];
    return values.every((v) => JSON.stringify(v) === JSON.stringify(firstValue)) ? firstValue : undefined;
  };

  // TODO: Implement setConnection and setConnections commands
  const handleChange = (_updatedConnection: Connection) => {};

  const handleGapChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, gap: value });
  };

  const handleShiftChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, shift: value });
  };

  const handleRiseChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rise: value });
  };

  const handleXOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, x: value });
  };

  const handleYOffsetChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, y: value });
  };

  const handleRotationChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, rotation: value });
  };

  const handleTurnChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, turn: value });
  };

  const handleTiltChange = (value: number) => {
    if (isSingle) handleChange({ ...connection!, tilt: value });
  };

  const commonGap = getCommonValue((c) => c.gap);
  const commonShift = getCommonValue((c) => c.shift);
  const commonRise = getCommonValue((c) => c.rise);
  const commonXOffset = getCommonValue((c) => c.x);
  const commonYOffset = getCommonValue((c) => c.y);
  const commonRotation = getCommonValue((c) => c.rotation);
  const commonTurn = getCommonValue((c) => c.turn);
  const commonTilt = getCommonValue((c) => c.tilt);

  return (
    <>
      {isSingle && (
        <>
          <TreeItem>
            <TreeContent>
              <Input label="Connecting Piece ID" value={connection!.connecting.piece} disabled />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input label="Connecting Port ID" value={connection!.connecting.port} disabled />
            </TreeContent>
          </TreeItem>
          {connection!.connecting.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input label="Connecting Design Piece ID" value={connection!.connecting.designPiece} disabled />
              </TreeContent>
            </TreeItem>
          )}
          <TreeItem>
            <TreeContent>
              <Input label="Connected Piece ID" value={connection!.connected.piece} disabled />
            </TreeContent>
          </TreeItem>
          <TreeItem>
            <TreeContent>
              <Input label="Connected Port ID" value={connection!.connected.port} disabled />
            </TreeContent>
          </TreeItem>
          {connection!.connected.designPiece && (
            <TreeItem>
              <TreeContent>
                <Input label="Connected Design Piece ID" value={connection!.connected.designPiece} disabled />
              </TreeContent>
            </TreeItem>
          )}
        </>
      )}
      {!isSingle && (
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">Editing {connections.length} connections simultaneously</p>
          </TreeContent>
        </TreeItem>
      )}
      <TreeItem>
        <TreeContent>
          <Stepper
            label={t("semio.connection.gap")}
            value={isSingle ? (connection!.gap ?? 0) : (commonGap ?? 0)}
            onChange={handleGapChange}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label={t("semio.connection.shift")}
            value={isSingle ? (connection!.shift ?? 0) : (commonShift ?? 0)}
            onChange={handleShiftChange}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label={t("semio.connection.rise")}
            value={isSingle ? (connection!.rise ?? 0) : (commonRise ?? 0)}
            onChange={handleRiseChange}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("semio.connection.rotation")}</label>
            <Slider
              value={[isSingle ? (connection!.rotation ?? 0) : (commonRotation ?? 0)]}
              onValueChange={([value]) => handleRotationChange(value)}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("semio.connection.turn")}</label>
            <Slider
              value={[isSingle ? (connection!.turn ?? 0) : (commonTurn ?? 0)]}
              onValueChange={([value]) => handleTurnChange(value)}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <div className="flex flex-col gap-1">
            <label className="text-xs">{t("semio.connection.tilt")}</label>
            <Slider
              value={[isSingle ? (connection!.tilt ?? 0) : (commonTilt ?? 0)]}
              onValueChange={([value]) => handleTiltChange(value)}
              startTransaction={startTransaction}
              finalizeTransaction={finalizeTransaction}
              abortTransaction={abortTransaction}
              min={-180}
              max={180}
              step={1}
            />
          </div>
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label="X Offset"
            value={isSingle ? (connection!.x ?? 0) : (commonXOffset ?? 0)}
            onChange={handleXOffsetChange}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
      <TreeItem>
        <TreeContent>
          <Stepper
            label="Y Offset"
            value={isSingle ? (connection!.y ?? 0) : (commonYOffset ?? 0)}
            onChange={handleYOffsetChange}
            startTransaction={startTransaction}
            finalizeTransaction={finalizeTransaction}
            abortTransaction={abortTransaction}
            step={0.1}
          />
        </TreeContent>
      </TreeItem>
    </>
  );
};

export const PortSection: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const isInDesignScope = useIsInDesignScope();
  if (!isInDesignScope) return null;
  return <PortSectionForm pieceGuid={pieceGuid} portGuid={portGuid} />;
};

const PortSectionForm: FC<{ pieceGuid: Guid; portGuid: Guid }> = ({ pieceGuid, portGuid }) => {
  const { t } = useTranslation();
  const design = useDesign() as Design;
  const kit = useKit() as Kit;

  const piece = (() => {
    try {
      return findPieceInDesign(design, pieceGuid);
    } catch {
      return null;
    }
  })();

  const type = piece?.type && typeof piece.type === "string" ? findTypeInKit(kit, piece.type) : null;
  const port = type?.ports?.find((p) => p.guid === portGuid);

  if (!piece || !type || !port) {
    return (
      <TreeItem label="Port" defaultOpen={true}>
        <TreeItem>
          <TreeContent>
            <p className="text-sm text-muted-foreground">Port not found</p>
          </TreeContent>
        </TreeItem>
      </TreeItem>
    );
  }

  return (
    <TreeItem label="Port" defaultOpen={true}>
      <Input label={t("semio.piece.id")} value={port.guid || "~default~"} disabled />
      {port.description && <Textarea label="Description" value={port.description} disabled />}
      {port.family && <Input label="Family" value={port.family} disabled />}
      {port.mandatory !== undefined && <Input label="Mandatory" value={port.mandatory ? "Yes" : "No"} disabled />}
      <Input label="Position" value={`(${port.point.x.toFixed(2)}, ${port.point.y.toFixed(2)}, ${port.point.z.toFixed(2)})`} disabled />
      <Input label="Direction" value={`(${port.direction.x.toFixed(2)}, ${port.direction.y.toFixed(2)}, ${port.direction.z.toFixed(2)})`} disabled />
      {port.compatibleFamilies &&
        port.compatibleFamilies.map((family: string) => (
          <TreeItem>
            <TreeContent>
              <Input label="Compatible Families" value={family} disabled />
            </TreeContent>
          </TreeItem>
        ))}
      {port.attributes &&
        port.attributes.map((attribute: any) => (
          <TreeItem>
            <TreeContent>
              <Input label={t("semio.design.attributes")} value={`${attribute.key}: ${attribute.value || "N/A"} ${attribute.unit && `(${attribute.unit})`}`} disabled />
            </TreeContent>
          </TreeItem>
        ))}
    </TreeItem>
  );
};
