// #region Header

// PageNavigation.tsx

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

import { ChevronLeft, ChevronRight } from "lucide-react";
import { FC } from "react";
import { Link } from "react-router";

interface NavigationLink {
  path: string;
  title: string;
  section: string;
}

interface PageNavigationProps {
  prev?: NavigationLink;
  next?: NavigationLink;
}

const PageNavigation: FC<PageNavigationProps> = ({ prev, next }) => {
  return (
    <div className="flex items-center justify-between gap-4 pt-8 mt-8 border-t border-border">
      <div className="flex-1">
        {prev && (
          <Link to={`/${prev.path}`} className="flex items-center gap-2 p-4 rounded-lg border border-border hover:bg-hover-panel hover:border-border transition-colors group">
            <ChevronLeft className="w-5 h-5 text-muted-foreground transition-colors" />
            <div className="flex flex-col items-start">
              <span className="text-xs text-muted-foreground uppercase tracking-wide">Previous</span>
              <span className="text-sm font-medium text-foreground">{prev.title}</span>
            </div>
          </Link>
        )}
      </div>
      <div className="flex-1 flex justify-end">
        {next && (
          <Link to={`/${next.path}`} className="flex items-center gap-2 p-4 rounded-lg border border-border hover:bg-hover-panel hover:border-border transition-colors group">
            <div className="flex flex-col items-end">
              <span className="text-xs text-muted-foreground uppercase tracking-wide">Next</span>
              <span className="text-sm font-medium text-foreground">{next.title}</span>
            </div>
            <ChevronRight className="w-5 h-5 text-muted-foreground transition-colors" />
          </Link>
        )}
      </div>
    </div>
  );
};

export default PageNavigation;
