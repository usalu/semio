import FormationEditor from './components/FormationEditor'

function App(): JSX.Element {
  const ipcHandle = (): void => window.electron.ipcRenderer.send('ping')

  return (
    <>
      {/* <div className="action">
        <a target="_blank" rel="noreferrer" onClick={ipcHandle}>
          Send IPC
        </a>
      </div> */}
      <div >
      <FormationEditor />
      </div>
    </>
  )
}

export default App
