from logging import info
from os import path, popen
import sys
from multiprocessing import Process, freeze_support
from time import sleep
from semio.backend.gateway import main as gatewayMain
from semio.backend.assembler import main as assemblerMain
from semio.backend.manager import main as managerMain
from semio.extensions.grasshopper import main as grasshopperMain

#https://stackoverflow.com/questions/48210090/how-to-use-bundled-program-after-pyinstaller-add-binary
def resource_path(relative_path):
    """ Get absolute path to resource, works for dev and for PyInstaller """
    base_path = getattr(sys, '_MEIPASS', path.dirname(path.abspath(__file__)))
    return path.join(base_path, relative_path)

def restproxyProcess():
    popen(resource_path("restproxy.exe -grpc-server-endpoint localhost:2000"))

def main(*args):
    # https://docs.python.org/3.10/library/multiprocessing.html#multiprocessing.freeze_support
    freeze_support()
    restproxy = Process(name="semio rest proxy", target=restproxyProcess)
    restproxy.start()
    gateway = Process(name="semio gateway", target=gatewayMain)
    gateway.start()
    assembler = Process(name="semio assembler", target=assemblerMain)
    assembler.start()
    manager = Process(name="semio manager", target=managerMain)
    manager.start()
    grasshopper = Process(name="semio grasshopper extension", target=grasshopperMain)
    while not manager.is_alive():
        sleep(0.1)
    grasshopper.start()
    while not grasshopper.is_alive():
        sleep(2)
    info("All services were started. Hit CTRL + C to exit.")
    try:
        while True:
            input()
    except KeyboardInterrupt:
        sys.exit()

if __name__ == '__main__':
    main()