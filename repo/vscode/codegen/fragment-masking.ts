// #region 🔖Header

// 💻repo/vscode/codegen/fragment-masking.ts

// 2026 Ueli Saluz <ueli@semio-tech.de>

// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖License

// #region 🔖Requirements
// #endregion 🔖Requirements

// #endregion 🔖Header

import { ResultOf, DocumentTypeDecoration, TypedDocumentNode } from '@graphql-typed-document-node/core';
import { FragmentDefinitionNode } from 'graphql';
import { Incremental } from './graphql';

export type FragmentType<TDocumentType extends DocumentTypeDecoration<any, any>> = TDocumentType extends DocumentTypeDecoration<
  infer TType,
  any
>
  ? [TType] extends [{ ' $fragmentName'?: infer TKey }]
  ? TKey extends string
  ? { ' $fragmentRefs'?: { [key in TKey]: TType } }
  : never
  : never;

_documentNode: DocumentTypeDecoration<TType, any>,
  fragmentType: FragmentType<DocumentTypeDecoration<TType, any>>
): TType;
// return nullable if `fragmentType` is undefined
export function useFragmen<TType, any>,
fragmentType: FragmentType<DocumentTypeDecoration<TType, any>> | undefined
): TType | undefined;
// return nullable if `fragmentType` is nullable
export function useFragment<TType>(
  _documentNode: DocumentTypeDecoration<ntTypeDecoration<TType, any>> | null
): TType | null;
// return nullable if `fragmentType` is nullable or undefined
export function useFragment<TType>(
  _documentNode: DocumentTypeDecoration<TType, any>,
  fragmentType: Fragme
): TType | null | undefined;
// return array of non-nullable if `fragmentType` is array of non-nullable
export function useFragment<TType>(
  _documentNode: DocumentTypeDecoration<TType, any>,
  fragmentType: Array<FragmentType<DocumentTypeDecoration<TType, any>>>of nullable
export function useFragment<TType>(
    _documentNode: DocumentTypeDecoration<TType, any>,
    fragmentType: Array<FragmentType<DocumentTypeDecoration<TType, any>>> | null | undefined
  ): Array<TType> | null | undefined;
_documentNode: DocumentTypeDecoration<TType, any>,
  fragmentType: ReadonlyArray<FragmentType<DocumentTypeDecoration<TType, any>>>
): ReadonlyArray<TType>;
// return readonly array of nullable if `fragmentType` is array of nullable
export function useFe: ReadonlyArray<FragmentType<DocumentTypeDecoration<TType, any>>> | null | undefined
): ReadonlyArray<TType> | null | undefined;
export function useFragment<TType>(
  _documentNode: DocumentTypeDecoration<TType, any>,
  fragmentType: FragmentType<DocumentTypeDecoration<TType, aay<FragmentType<DocumentTypeDecoration<TType, any>>> | null | undefined
): TType | Array<TType> | ReadonlyArray<TType> | null | undefined {
  return fragmentType as any;
}

export function makeFragmentData<
  F extends DocumentTypeDecoration<any, any>,
  FT extends ResultOf<F>
>(data: FT, _fragment: F): FragmentType<F> {
  return data as FragmentType<F>;
}
export function isFragmentReady<TQuery, TFrag>(
  queryNode: DocumentTypeDecoration<TQuery, any>,
  fragmentNode: TypedDocumentNode<TFrag>,
  data: FragmentType<TypedDocumentNode<Incremental<TFrag>, any>> | null | undefined
): data is FragmentType<typeof fragmentNode> {
  const deferredFields = (queryNode as { __meta__?: { deferredFields: Record<string, (keyof TFrag)[]> } }).__meta__
    ?.deferredFields;

  if (!deferredFields) return true;

  const fragDef = fragmentNode.definitions[0] as FragmentDefinitionNode | undefined;
  const fragName = fragDef?.name?.value;

  const fields = (fragName && deferredFields[fragName]) || [];
  return fields.length > 0 && fields.every(field => data && field in data);
}
