// #region Header

// Tree.tsx

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

// #endregion TODOs

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@semio/js/components/ui/Collapsible'
import { ChevronDown, ChevronRight } from 'lucide-react'
import { createContext, FC, ReactNode, useContext, useState } from 'react'

const TreeContext = createContext<{ level: number }>({ level: 0 })

export interface TreeSectionAction {
    icon: ReactNode
    onClick: () => void
    title?: string
}

interface TreeSectionProps {
    label: string
    icon?: ReactNode
    children?: ReactNode
    defaultOpen?: boolean
    className?: string
    actions?: TreeSectionAction[]
}

interface TreeItemProps {
    label?: ReactNode
    icon?: ReactNode
    children?: ReactNode
    onClick?: () => void
    className?: string
    isSelected?: boolean
    isHighlighted?: boolean
}

export const TreeSection: FC<TreeSectionProps> = ({ label, icon, children, defaultOpen = true, className = '', actions = [] }) => {
    const [open, setOpen] = useState(defaultOpen)
    const [isHovered, setIsHovered] = useState(false)
    const { level } = useContext(TreeContext)
    const indentStyle = { paddingLeft: `${level * 1.25}rem` }
    const hasChildren = Boolean(children)

    if (!hasChildren) {
        return (
            <TreeContext.Provider value={{ level: level + 1 }}>
                <div
                    className={`flex items-center gap-2 py-1.5 px-2 hover:bg-muted select-none overflow-hidden group ${className}`}
                    style={indentStyle}
                    onMouseEnter={() => setIsHovered(true)}
                    onMouseLeave={() => setIsHovered(false)}
                >
                    <div className="w-[14px] flex-shrink-0" />
                    {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                    <span className="flex-1 text-sm text-muted-foreground uppercase tracking-wide truncate">{label}</span>
                    {actions.length > 0 && (
                        <div className="flex items-center gap-1">
                            {actions.map((action, index) => (
                                <button
                                    key={index}
                                    className={`p-1 rounded-sm transition-opacity hover:bg-muted-foreground/10 ${isHovered ? 'opacity-100' : 'opacity-30 group-hover:opacity-60'}`}
                                    onClick={(e) => {
                                        e.preventDefault()
                                        e.stopPropagation()
                                        action.onClick()
                                    }}
                                    title={action.title}
                                >
                                    {action.icon}
                                </button>
                            ))}
                        </div>
                    )}
                </div>
            </TreeContext.Provider>
        )
    }

    return (
        <TreeContext.Provider value={{ level: level + 1 }}>
            <Collapsible open={open} onOpenChange={setOpen}>
                <CollapsibleTrigger asChild>
                    <div
                        className={`flex items-center gap-2 py-1.5 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden group ${className}`}
                        style={indentStyle}
                        onMouseEnter={() => setIsHovered(true)}
                        onMouseLeave={() => setIsHovered(false)}
                    >
                        {open ? (
                            <ChevronDown size={14} className="flex-shrink-0" />
                        ) : (
                            <ChevronRight size={14} className="flex-shrink-0" />
                        )}
                        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                        <span className="flex-1 text-sm text-muted-foreground uppercase tracking-wide truncate">{label}</span>
                        {actions.length > 0 && (
                            <div className="flex items-center gap-1">
                                {actions.map((action, index) => (
                                    <button
                                        key={index}
                                        className={`p-1 rounded-sm transition-opacity hover:bg-muted-foreground/10 ${isHovered ? 'opacity-100' : 'opacity-30 group-hover:opacity-60'}`}
                                        onClick={(e) => {
                                            e.preventDefault()
                                            e.stopPropagation()
                                            action.onClick()
                                        }}
                                        title={action.title}
                                    >
                                        {action.icon}
                                    </button>
                                ))}
                            </div>
                        )}
                    </div>
                </CollapsibleTrigger>
                <CollapsibleContent>{children}</CollapsibleContent>
            </Collapsible>
        </TreeContext.Provider>
    )
}

export const TreeItem: FC<TreeItemProps> = ({
    label,
    icon,
    children,
    onClick,
    className = '',
    isSelected = false,
    isHighlighted = false
}) => {
    const { level } = useContext(TreeContext)
    const indentStyle = { paddingLeft: `${level * 1.25}rem` }
    const baseClasses = "flex items-center gap-2 py-1 px-2 hover:bg-muted cursor-pointer select-none overflow-hidden"
    const stateClasses = `${isSelected ? 'bg-accent' : ''} ${isHighlighted ? 'bg-accent/50' : ''}`
    const itemClasses = `${baseClasses} ${stateClasses} ${className}`

    if (children) {
        return (
            <TreeContext.Provider value={{ level: level + 1 }}>
                {label && (
                    <div
                        className={itemClasses}
                        style={indentStyle}
                        onClick={onClick}
                    >
                        {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
                        <span className="flex-1 text-sm font-normal truncate">{label}</span>
                    </div>
                )}
                <div className="px-2 pb-1">
                    {children}
                </div>
            </TreeContext.Provider>
        )
    }

    if (!label) {
        return children || null
    }

    return (
        <div
            className={itemClasses}
            style={indentStyle}
            onClick={onClick}
        >
            {icon && <span className="flex items-center justify-center flex-shrink-0">{icon}</span>}
            <span className="flex-1 text-sm font-normal truncate">{label}</span>
        </div>
    )
}

export const Tree: FC<{ children: ReactNode; className?: string }> = ({ children, className = '' }) => {
    return (
        <TreeContext.Provider value={{ level: 0 }}>
            <div className={`w-full ${className}`}>{children}</div>
        </TreeContext.Provider>
    )
}
