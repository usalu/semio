// #region 🔖Header

// [💻semio/assets/repo/some/folder/file_fixed.tsx](semiorepo://file/semio/assets/repo/some/folder/file_fixed.tsx)

// 2025 Test User <test@test.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// A fixed TypeScript component for testing.

// #endregion 🔖Header

// #region 🔖Types
// [🔖semio/assets/repo/some/folder/file_fixed.tsx#Types](semiorepo://section/semio/assets/repo/some/folder/file_fixed.tsx/TYPES)
// Type definitions for the fixed component.
// Types MUST be exported when used externally.

// Properties of a fixed component.
// FixedType MUST have a name and value.
interface FixedType {
  name: string;
  value: number;
}

// Kind alternatives for fixed types.
// FixedKind MUST be one of alpha or beta.
type FixedKind = "alpha" | "beta";

// #endregion 🔖Types

// #region 🔖Components
// [🔖semio/assets/repo/some/folder/file_fixed.tsx#Components](semiorepo://section/semio/assets/repo/some/folder/file_fixed.tsx/COMPONENTS)
// Rendering components for fixed types.
// Components MUST accept FixedType props.

/**
 * Renders a fixed component by returning its name.
 * FixedComponent MUST return the name property.
 * [🛠️semio/assets/repo/some/folder/file_fixed.tsx#Components§FixedComponent](semiorepo://definition/semio/assets/repo/some/folder/file_fixed.tsx/COMPONENTS/FIXED-COMPONENT)
 **/
export function FixedComponent(props: FixedType): string {
  return props.name;
}

// #endregion 🔖Components
