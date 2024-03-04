import { useState, useRef } from 'react'
import { KBarProvider } from 'kbar'
import FormationEditor from './components/FormationEditor'
import CommandBar from './components/CommandBar'
import { Kit } from '@renderer/semio'
function App(): JSX.Element {
    const [kit, setKit] = useState<Kit | null>(null)
    const ipcHandle = (): void => window.electron.ipcRenderer.send('select-directory')
    const formationEditorRef = useRef(null)
    const actions = [
        {
            id: 'open-kit',
            name: 'Open Kit',
            shortcut: ['$mod+o'],
            keywords: 'new',
            section: 'Files',
            perform: () => {
                ipcHandle()
                window.electron.ipcRenderer.on('directory-selected', (event, path) => {
                    // not implemented yet
                    // setKit(kit)
                })
            }
        },
        {
            id: 'reload-kit',
            name: 'Reload Kit',
            shortcut: ['$mod+r'],
            keywords: 'update',
            section: 'Files',
            perform: () => {}
        },
        {
            id: 'zoom-to-fit',
            name: 'Zoom to Fit',
            shortcut: ['$mod+t'],
            keywords: 'formation',
            section: 'Navigation',
            perform: () => {
                if (formationEditorRef.current) {
                    formationEditorRef.current.zoomToFit()
                }
            }
        }
    ]
    return (
        <KBarProvider actions={actions}>
            <CommandBar />
            <FormationEditor ref={formationEditorRef} />
        </KBarProvider>
    )
}
export default App
